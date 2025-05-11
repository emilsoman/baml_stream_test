use baml_runtime::BamlRuntime;
use baml_runtime::FunctionResult;
use baml_types::BamlMap;
use baml_types::BamlValue;
use colored::*;
use std::path::PathBuf;

fn main() {
    stream_test_return_int();
    println!("{}", "--------------------------------".yellow().bold());
    stream_test_return_string();
    println!("{}", "--------------------------------".yellow().bold());
    stream_test_return_string2();
    println!("{}", "--------------------------------".yellow().bold());
    stream_test_create_persona();
    println!("{}", "--------------------------------".yellow().bold());
}

fn stream_test_return_int() {
    // Example parameters
    // {user: {name: "John Doe"}}
    let mut params = BamlMap::new();
    params.insert(
        "name".to_string(),
        BamlValue::String("John Doe".to_string()),
    );

    stream("ReturnInt".to_string(), params);
}

fn stream_test_return_string() {
    // Example parameters
    // {user: {name: "John Doe"}}
    let mut params = BamlMap::new();
    params.insert(
        "name".to_string(),
        BamlValue::String("John Doe".to_string()),
    );

    stream("ReturnString".to_string(), params);
}

fn stream_test_return_string2() {
    // Example parameters
    // {user: {name: "John Doe"}}
    let mut params = BamlMap::new();
    params.insert(
        "name".to_string(),
        BamlValue::String("John Doe".to_string()),
    );

    stream("ReturnString2".to_string(), params);
}

fn stream_test_create_persona() {
    // Example parameters
    // {user: {name: "John Doe"}}
    let mut params = BamlMap::new();
    let mut person = BamlMap::new();
    person.insert(
        "name".to_string(),
        BamlValue::String("John Doe".to_string()),
    );
    person.insert("age".to_string(), BamlValue::Int(30));
    params.insert("person".to_string(), BamlValue::Map(person));

    stream("CreatePersona".to_string(), params);
}

fn stream(function_name: String, params: BamlMap<String, BamlValue>) {
    println!(
        "{} {}",
        "Running function:".green(),
        function_name.cyan().bold()
    );
    println!(
        "{} {}",
        "With params:".green(),
        format!("{:?}", params).cyan()
    );

    // Create a new BAML runtime from the current directory
    let runtime = BamlRuntime::from_directory(&PathBuf::from("."), std::env::vars().collect())
        .expect("Failed to create BAML runtime");

    // Create a context manager
    let ctx_manager = runtime.create_ctx_manager(BamlValue::String("rust".to_string()), None);

    // Define the event handler
    let on_event = |r: FunctionResult| {
        println!("{}", "Event received:".blue());
        match r.parsed() {
            Some(Ok(content)) => {
                println!("{} {}", "Content:".green(), format!("{:?}", content).cyan());
            }
            Some(Err(e)) => {
                println!("{} {}", "Error:".red(), format!("{:?}", e).red());
            }
            None => {
                println!("{}", "No content".yellow());
            }
        }
    };

    // Run the stream
    let result = runtime.stream_function(function_name, &params, &ctx_manager, None, None, None);

    match result {
        Ok(mut stream) => {
            let (result, _trace_id) = stream.run_sync(Some(on_event), &ctx_manager, None, None);
            match result {
                Ok(f) => println!(
                    "{} {}",
                    "Stream completed successfully:".green(),
                    format!("{:?}", f).cyan()
                ),
                Err(e) => println!("{} {}", "Stream error:".red(), format!("{:?}", e).red()),
            }
        }
        Err(e) => println!(
            "{} {}",
            "Failed to create stream:".red(),
            format!("{:?}", e).red()
        ),
    }
}
