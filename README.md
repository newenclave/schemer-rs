# schemer

### what is this?

It's a json schema (https://json-schema.org/) / json value generator from an own format.

### why?

it's written in Rust as an "helloworld" pet project. So with this project I wanted to dive into Rust.

### how does it work? 

You give schemer file, it return json schema. But for now there is no json generator. So it just parses the own schema and generates the same schema. 

### depends?

On nothing.

### only json schema?

Not exactly. Probably some another features will be added later.

## format description?

Sure. It's simple.
```
object_name(options): type[] = value
```
Here: 
`object_name` - any valid ident value or string with double quotes.
`(option)` - option list belongs to field
`type` - one of `string`, `integer`, `floating`, `Boolean`, `object`. Probably aliases will be added soon. Any field can be an array. `[]`

### Examples?

Here. Some examples could be invalid. I'm in process to writing/changing the code

An empty object:
```
main: object  {}
```

Some values:
```
main: object {
    i: integer = 1,
    f: floating
    s: string[] = ["Hello", "World!"]
}
```

Neted objects:
```
main: object {
    obj: object {
        i: integer
    }
}
```

Defining values: 
```
main: object {
    obj: object {
        i: integer = 42
    }
    obj2: object[] {
        s: string = "",
        b: boolean = true
    } = [ {"s": "1"}, {"s": "2"}, {s: "3"} ]
} 
```

Options: 
```
main(main_option): object {
    obj(readonly): object {
        i: integer 1..100 = 50
    }
    obj2(some_option: "some_string_value"): object[] {
        s: string = ""
    } = [ {"s": "1"}, {"s": "2"}, {s: "3"} ]
} 
```

Also i'm gonna add eamples. See `test_data` directory.
