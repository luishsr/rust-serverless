use wasmtime::*;
use serde_json::Value;

pub fn execute(code: &str, function_name: &str, inputs: &[Value]) -> Result<Value, Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let module = Module::new(&engine, code)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    let func = instance.get_func(&mut store, function_name)
        .ok_or_else(|| format!("Function '{}' not found in module", function_name))?;

    let func_ty = func.ty(&store);
    let params: Vec<_> = func_ty.params().collect();
    let results: Vec<_> = func_ty.results().collect();

    println!("Executing function: {}", function_name);
    println!("Inputs: {:?}", inputs);
    println!("Params: {:?}", params);
    println!("Results: {:?}", results);

    if params.len() != inputs.len() {
        return Err(format!(
            "Function '{}' expected {} arguments, but got {}",
            function_name, params.len(), inputs.len()
        ).into());
    }

    let mut wasm_inputs = Vec::new();
    for (param, input) in params.iter().zip(inputs.iter()) {
        let value = match (param, input) {
            (ValType::I32, Value::Number(n)) => Val::I32(n.as_i64().ok_or("Invalid i32")? as i32),
            (ValType::F32, Value::Number(n)) => Val::F32((n.as_f64().ok_or("Invalid f32")? as f32).to_bits()),
            (ValType::I64, Value::Number(n)) => Val::I64(n.as_i64().ok_or("Invalid i64")?),
            (ValType::F64, Value::Number(n)) => Val::F64(n.as_f64().ok_or("Invalid f64")?.to_bits()),
            _ => return Err(format!("Unsupported parameter type: {:?}", param).into()),
        };
        wasm_inputs.push(value);
    }

    let mut wasm_results = vec![Val::I32(0); results.len()];
    func.call(&mut store, &wasm_inputs, &mut wasm_results)?;

    if wasm_results.len() > 1 {
        return Err("Multiple return values are not supported yet".into());
    }

    let result = match wasm_results.get(0) {
        Some(Val::I32(v)) => Value::Number((*v).into()),
        Some(Val::F32(v)) => Value::String(f32::from_bits(*v).to_string()),
        Some(Val::I64(v)) => Value::Number((*v).into()),
        Some(Val::F64(v)) => Value::String(f64::from_bits(*v).to_string()),
        None => Value::Null,
        _ => return Err("Unsupported return type".into()),
    };

    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_add_function() {
        let wasm_code = r#"
    (module
      (func (export "add") (param i32 i32) (result i32)
        local.get 1
        local.get 0
        i32.add
      )
    )
    "#;

        let result = execute(wasm_code, "add", &[Value::Number(3.into()), Value::Number(7.into())])
            .expect("Execution failed");
        assert_eq!(result, Value::Number(10.into()));
    }

    #[test]
    fn test_execute_valid_wasm() {
        let wasm_code = r#"
        (module
          (func (export "main") (result i32)
            (i32.const 42)))
        "#;

        let result = execute(wasm_code, "main", &[]).expect("Execution failed");
        assert_eq!(result, Value::Number(42.into()));
    }

    #[test]
    fn test_execute_with_input() {
        let wasm_code = r#"
        (module
          (func (export "main") (param i32) (result i32)
            local.get 0))
        "#;

        let result = execute(wasm_code, "main", &[Value::Number(21.into())]).expect("Execution failed");
        assert_eq!(result, Value::Number(21.into()));
    }
}
