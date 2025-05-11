client GPT4 {
    provider openai
    options {
        model gpt-4o-mini
        api_key env.OPENAI_API_KEY
    }
}

function ReturnInt(name: string) -> int {
  client GPT4
  prompt #"
    {{ctx.output_format}}

    How long is the user's name?
    {{name}}
  "#
}

function ReturnString(name: string) -> string {
  client GPT4
  prompt #"
  Return just the first 2 characters of {{name}}
  "#
}

class MyString {
  value string
}

function ReturnString2(name: string) -> MyString {
  client GPT4
  prompt #"
  Return just the first 2 characters of {{name}}
  {{ ctx.output_format }}
  "#
}

class Person {
    name string
    age int
}

class Persona {
    value string
}

function CreatePersona(person: Person) -> Persona {
    client GPT4
    prompt #"
        Create a persona for the following person in 50 characters or less:

        Person:
        {{ person }}

        {{ ctx.output_format }}
    "#
}