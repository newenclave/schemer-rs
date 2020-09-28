# schemer


### what is this?

It's a json schema (https://json-schema.org/) / json value generator from an own format.

### why?

it's written in Rust as an "helloworld" pet project. So with this project I wanted to dive into Rust.

### how does it work?

You give schemer file, it returns json schema or json values.

### depends?

On nothing.

### only json schema?

Not exactly. Probably some another features will be added later.

## format description?

Sure. It's simple.

```schemer
object_name(options): type[] = value
```

Here:
`object_name` - any valid ident value or string with double quotes.  
`(option)` - option list belongs to field  
`type` - one of `string`, `integer`, `floating`, `boolean`, `object`. Probably aliases will be added soon. Any field can be an array. `[]`  

### To run

```bash
$ ./schemer <path_to_schemer_file> [[json_value | json_schema | schemer] <shift_value: 1, 2, 3, 4...>]
```

### Examples?

Here. Some examples could be invalid. I'm in process to writing/changing the code

An empty object:

```schemer
main: object  {}
```

Some values:

```schemer
main: object {
    i: integer = 1,
    f: floating
    s: string[] = ["Hello", "World!"]
}
```

output json_schema:

```json
{
  "properties": {
    "i": {
      "type": "integer"
    },
    "s": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "f": {
      "type": "number"
    }
  },
  "type": "object"
}
```

output json_value:

```json
{
  "f": 0,
  "s": ["Hello", "World!"],
  "i": 1
}
```

Neted objects:

```schemer
main: object {
    obj: object {
        i: integer
    }
}
```

Defining values:

```schemer
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

```schemer
main(main_option): object {
    obj(readonly): object {
        i: integer 1..100 = 50
    }
    obj2(some_option: "some_string_value"): object[] {
        s: string = ""
    } = [ {"s": "1"}, {"s": "2"}, {s: "3"} ], 
    obj3(json_opt: { field1: 1, field2: "!", field3: {} }): boolean;
}

```

Enums:

```schemer
main: object {
    i: integer enum { 1, 3, 5, 7, 9 }        # defaul value is 1
    s: string enum { "one", "two", "three" } # defaul value is "one"
    f: floating enum { 0.5, 1, 1.5 }         # defaul value is 0.5
}
```

json_schema output:

```json
{
  "properties": {
    "i": {
      "enum": [1, 3, 5, 7, 9],
      "type": "integer"
    },
    "f": {
      "type": "number",
      "enum": [0.5, 1, 1.5]
    },
    "s": {
      "type": "string",
      "enum": ["one", "two", "three"]
    }
  },
  "type": "object"
}
```

json_value output:

```json
{
  "s": "one",
  "i": 1,
  "f": 0.5
}
```

Options:

```schemer
root: object {
    flag(readonly, required): boolean;
    iro(readonly): integer;
    sro(readonly): string;
    aro(readonly): object[] {
        nro: boolean;
        some_data(readonly): string;
    } = [{}, {}, {}]
    fro: floating[] = [0, 0.5, 1, 1.5, 2]
}
```

json_schema:

```json
{
  "required": ["flag"],
  "type": "object",
  "properties": {
    "sro": {
      "type": "string",
      "readonly": true
    },
    "fro": {
      "items": {
        "type": "number"
      },
      "type": "array"
    },
    "aro": {
      "items": {
        "type": "object",
        "properties": {
          "some_data": {
            "type": "string",
            "readonly": true
          },
          "nro": {
            "type": "boolean"
          }
        }
      },
      "type": "array",
      "readonly": true
    },
    "flag": {
      "type": "boolean",
      "readonly": true
    },
    "iro": {
      "readonly": true,
      "type": "integer"
    }
  }
}
```

json vaue:

```json
{
  "sro": "",
  "iro": 0,
  "aro": [
    {
      "nro": false,
      "some_data": ""
    }, {
      "nro": false,
      "some_data": ""
    }, {
      "nro": false,
      "some_data": ""
    }
  ],
  "fro": [0, 0.5, 1, 1.5, 2],
  "flag": false
}
```

Any:

```schemer
main: object {
    any_object: any = {         # it's possible to define any
        data: "string value",
        i: 1000,
        f: 0.5,
        a: true,
        b: false,
        n: null,
        aa: {
            some_nested: {}
        }
    };
    any_int: any = 10,
    any_float: any = 10.5,
    any_string: any = "this is an any string"
    eny_empty_array: any = []
    valid_json_any: any = {
        "data": "data",
        "i": 100,
        "b": [{}, 1, ""]
    }
    valid_any_array: any = [1, 1.5, "string", {}];  # any is a special keyword, not a typename
    #invalid_any_array: any[] = [];                 # any array cannot be defined with [] 
}
```

schema:

```json
{
  "type": "object",
  "properties": {
    "any_object": {
      "properties": {
        "b": {
          "type": "boolean"
        },
        "n": {},
        "f": {
          "type": "number"
        },
        "a": {
          "type": "boolean"
        },
        "aa": {
          "type": "object",
          "properties": {
            "some_nested": {
              "properties": {},
              "type": "object"
            }
          }
        },
        "data": {
          "type": "string"
        },
        "i": {
          "type": "integer"
        }
      },
      "type": "object"
    },
    "any_int": {
      "type": "integer"
    },
    "any_string": {
      "type": "string"
    },
    "valid_any_array": {
      "type": "array"
    },
    "eny_empty_array": {
      "type": "array"
    },
    "any_float": {
      "type": "number"
    },
    "valid_json_any": {
      "type": "object",
      "properties": {
        "b": {
          "type": "array"
        },
        "i": {
          "type": "integer"
        },
        "data": {
          "type": "string"
        }
      }
    }
  }
}
```

value:

```json
{
  "any_string": "this is an any string",
  "any_float": 10.5,
  "eny_empty_array": [],
  "valid_json_any": {
    "b": [
      {}, 1, ""
    ],
    "i": 100,
    "data": "data"
  },
  "valid_any_array": [
    1, 1.5, "string", {}
  ],
  "any_object": {
    "i": 1000,
    "f": 0.5,
    "n": null,
    "data": "string value",
    "b": false,
    "aa": {
      "some_nested": {}
    },
    "a": true
  },
  "any_int": 10
}
```

#### Aliases

It's possible to define aliases for any type supported.

```schemer
mod examples.rule

# aliases are just types definitions.
# you can also specify defaults for the type defined with "alias"

alias rule_type: string enum { # string enum with default = "skip"
  "skip", "allow", "deny"
}
alias weight: integer 1..10 = 5 # integer interval with default = 5
alias rule: object {
  name: string,
  type: rule_type = "skip"
}

rules: object[] {
  enabled: boolean = false
  w: weight
  r: rule
} = [
    {
        "enabled": true,
        "w": 10,
        "r": {
            "name": "deny any",
            "type": "deny"
        }
    }
]
```

json_schema: 

```json
{ 
   "properties":{ 
      "rules":{ 
         "type":"array",
         "items":{ 
            "properties":{ 
               "w":{ 
                  "type":"integer",
                  "maximum":10,
                  "minimum":1
               },
               "r":{ 
                  "type":"object",
                  "properties":{ 
                     "type":{ 
                        "enum":[ 
                           "skip",
                           "allow",
                           "deny"
                        ],
                        "type":"string"
                     },
                     "name":{ 
                        "type":"string"
                     }
                  }
               },
               "enabled":{ 
                  "type":"boolean"
               }
            },
            "type":"object"
         }
      }
   },
   "type":"object"
}
```

json_value:

```json
{ 
   "rules":[ 
      { 
         "w":10,
         "enabled":true,
         "r":{ 
            "type":"deny",
            "name":"deny any"
         }
      }
   ]
}
```

Also i'm gonna add examples. See `test_data` directory.

#### Thanks 

To all users of https://t.me/rust_beginners_ru

#### Some tasks to do

- [x] remove all the panics. Parser should work with Result, not with panics
- [x] add `enum` for strings, numbers
- [ ] add `pattern` for strings
- [x] remove `unused` and clean the code out of unused
- [x] adding json generators
- [x] adding json generators for `any`
- [x] adding negative numbers
- [ ] removing all copy-paste
- [?] aliases
