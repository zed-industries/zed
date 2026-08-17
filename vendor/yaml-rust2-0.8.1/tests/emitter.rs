use yaml_rust2::{YamlEmitter, YamlLoader};

#[allow(clippy::similar_names)]
#[test]
fn test_emit_simple() {
    let s = "
# comment
a0 bb: val
a1:
    b1: 4
    b2: d
a2: 4 # i'm comment
a3: [1, 2, 3]
a4:
    - [a1, a2]
    - 2
";

    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];
    let mut writer = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut writer);
        emitter.dump(doc).unwrap();
    }
    println!("original:\n{s}");
    println!("emitted:\n{writer}");
    let docs_new = match YamlLoader::load_from_str(&writer) {
        Ok(y) => y,
        Err(e) => panic!("{}", e),
    };
    let doc_new = &docs_new[0];

    assert_eq!(doc, doc_new);
}

#[test]
fn test_emit_complex() {
    let s = r"
catalogue:
  product1: &coffee   { name: Coffee,    price: 2.5  ,  unit: 1l  }
  product2: &cookies  { name: Cookies!,  price: 3.40 ,  unit: 400g}

products:
  *coffee :
    amount: 4
  *cookies :
    amount: 4
  [1,2,3,4]:
    array key
  2.4:
    real key
  true:
    bool key
  {}:
    empty hash key
            ";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];
    let mut writer = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut writer);
        emitter.dump(doc).unwrap();
    }
    let docs_new = match YamlLoader::load_from_str(&writer) {
        Ok(y) => y,
        Err(e) => panic!("{}", e),
    };
    let new_doc = &docs_new[0];
    assert_eq!(doc, new_doc);
}

#[test]
fn test_emit_avoid_quotes() {
    let s = r#"---
a7: 你好
boolean: "true"
boolean2: "false"
date: 2014-12-31
empty_string: ""
empty_string1: " "
empty_string2: "    a"
empty_string3: "    a "
exp: "12e7"
field: ":"
field2: "{"
field3: "\\"
field4: "\n"
field5: "can't avoid quote"
float: "2.6"
int: "4"
nullable: "null"
nullable2: "~"
products:
  "*coffee":
    amount: 4
  "*cookies":
    amount: 4
  ".milk":
    amount: 1
  "2.4": real key
  "[1,2,3,4]": array key
  "true": bool key
  "{}": empty hash key
x: test
y: avoid quoting here
z: string with spaces"#;

    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];
    let mut writer = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut writer);
        emitter.dump(doc).unwrap();
    }

    assert_eq!(s, writer, "actual:\n\n{writer}\n");
}

#[test]
fn emit_quoted_bools() {
    let input = r#"---
string0: yes
string1: no
string2: "true"
string3: "false"
string4: "~"
null0: ~
[true, false]: real_bools
[True, TRUE, False, FALSE, y,Y,yes,Yes,YES,n,N,no,No,NO,on,On,ON,off,Off,OFF]: false_bools
bool0: true
bool1: false"#;
    let expected = r#"---
string0: "yes"
string1: "no"
string2: "true"
string3: "false"
string4: "~"
null0: ~
? - true
  - false
: real_bools
? - "True"
  - "TRUE"
  - "False"
  - "FALSE"
  - y
  - Y
  - "yes"
  - "Yes"
  - "YES"
  - n
  - N
  - "no"
  - "No"
  - "NO"
  - "on"
  - "On"
  - "ON"
  - "off"
  - "Off"
  - "OFF"
: false_bools
bool0: true
bool1: false"#;

    let docs = YamlLoader::load_from_str(input).unwrap();
    let doc = &docs[0];
    let mut writer = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut writer);
        emitter.dump(doc).unwrap();
    }

    assert_eq!(
        expected, writer,
        "expected:\n{expected}\nactual:\n{writer}\n",
    );
}

#[test]
fn test_empty_and_nested() {
    test_empty_and_nested_flag(false);
}

#[test]
fn test_empty_and_nested_compact() {
    test_empty_and_nested_flag(true);
}

fn test_empty_and_nested_flag(compact: bool) {
    let s = if compact {
        r"---
a:
  b:
    c: hello
  d: {}
e:
  - f
  - g
  - h: []"
    } else {
        r"---
a:
  b:
    c: hello
  d: {}
e:
  - f
  - g
  -
    h: []"
    };

    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];
    let mut writer = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut writer);
        emitter.compact(compact);
        emitter.dump(doc).unwrap();
    }

    assert_eq!(s, writer);
}

#[test]
fn test_nested_arrays() {
    let s = r"---
a:
  - b
  - - c
    - d
    - - e
      - f";

    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];
    let mut writer = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut writer);
        emitter.dump(doc).unwrap();
    }
    println!("original:\n{s}");
    println!("emitted:\n{writer}");

    assert_eq!(s, writer);
}

#[test]
fn test_deeply_nested_arrays() {
    let s = r"---
a:
  - b
  - - c
    - d
    - - e
      - - f
      - - e";

    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];
    let mut writer = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut writer);
        emitter.dump(doc).unwrap();
    }
    println!("original:\n{s}");
    println!("emitted:\n{writer}");

    assert_eq!(s, writer);
}

#[test]
fn test_nested_hashes() {
    let s = r"---
a:
  b:
    c:
      d:
        e: f";

    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];
    let mut writer = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut writer);
        emitter.dump(doc).unwrap();
    }
    println!("original:\n{s}");
    println!("emitted:\n{writer}");

    assert_eq!(s, writer);
}
