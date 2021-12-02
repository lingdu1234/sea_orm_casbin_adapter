# 完整测试用例/test cases

## step 1  <add_policy>

```rust
        e.add_policy(vec!["am".to_string(), "ad".to_string(), "rw".to_string()])
            .await
            .unwrap();
        e.add_policy(vec!["a".to_string(), "b".to_string(), "c".to_string()])
            .await
            .unwrap();
```

log

```log
Dec 02 09:19:34.440 INFO Policy Management, Event: Type: AddPolicy, Assertion: p::p,  Data: "am, ad, rw"
Dec 02 09:19:34.448 INFO Policy Management, Event: Type: AddPolicy, Assertion: p::p,  Data: "a, b, c"
test test::add_policy ... ok
```

result

```
+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
|  2 | p     | a    | b    | c    |      |      |      |
+----+-------+------+------+------+------+------+------+
```

## step 2  <add_policies>

```rust
e.add_policies(vec![
            vec!["bm".to_string(), "bd".to_string(), "rw".to_string()],
            vec!["bm".to_string(), "be".to_string(), "r".to_string()],
        ])
        .await
        .unwrap();
```

log

```log
Dec 02 09:20:44.025 INFO Policy Management, Event: Type: AddPolicies, Assertion: p::p, Added: 2
test test::add_policies ... ok
```

result

```
+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
|  2 | p     | a    | b    | c    |      |      |      |
|  3 | p     | bm   | bd   | rw   |      |      |      |
|  4 | p     | bm   | be   | r    |      |      |      |
+----+-------+------+------+------+------+------+------+
```

## step 3  <add_named_policy>

```rust
 e.add_named_policy(
            "g",
            vec!["cm".to_string(), "cd".to_string()],
        )
        .await
        .unwrap();
```

result

```
+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
|  2 | p     | a    | b    | c    |      |      |      |
|  3 | p     | bm   | bd   | rw   |      |      |      |
|  4 | p     | bm   | be   | r    |      |      |      |
|  5 | g     | cm   | cd   |      |      |      |      |
+----+-------+------+------+------+------+------+------+
```

## step 4  <add_named_policies>

```rust
e.add_named_policies(
    "g",
    vec![
        vec!["dm".to_string(), "dd".to_string()],
        vec!["dm".to_string(), "ee".to_string()],
    ],
)
.await
.unwrap();
```

```log
Dec 02 09:23:31.355 INFO Policy Management, Event: Type: AddPolicies, Assertion: p::g, Added: 2
test test::add_named_policies ... ok
```

result

```
+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
|  2 | p     | a    | b    | c    |      |      |      |
|  3 | p     | bm   | bd   | rw   |      |      |      |
|  4 | p     | bm   | be   |      |      |      |      |
|  5 | g     | cm   | cd   |      |      |      |      |
|  6 | g     | dm   | dd   |      |      |      |      |
|  7 | g     | dm   | ee   |      |      |      |      |
+----+-------+------+------+------+------+------+------+
```

## step 5  <remove_policy>


```rust
e.remove_policy(vec!["a".to_string(), "b".to_string(), "c".to_string()])
            .await
            .unwrap();
```

log

```log
Dec 02 09:25:11.576 INFO Policy Management, Event: Type: RemovePolicy, Assertion: p::p, Data: "a, b, c"
test test::remove_policy ... ok
```

result: `id=2 is removed.`

```
+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
|  3 | p     | bm   | bd   | rw   |      |      |      |
|  4 | p     | bm   | be   | r    |      |      |      |
|  5 | g     | cm   | cd   |      |      |      |      |
|  6 | g     | dm   | dd   |      |      |      |      |
|  7 | g     | dm   | ee   |      |      |      |      |
+----+-------+------+------+------+------+------+------+
```

## step 6  <remove_policies>

```rust
e.remove_policies(vec![
            vec!["bm".to_string(), "bd".to_string(), "rw".to_string()],
            vec!["bm".to_string(), "be".to_string(), "r".to_string()],
        ])
        .await
        .unwrap();
```

log

```log
Dec 02 09:27:16.990 INFO Policy Management, Event: Type: RemovePolicies, Assertion: p::p, Removed: 2
test test::remove_policies ... ok
```

result `id=3 and id=4 is removed`

```
+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
|  5 | g     | cm   | cd   |      |      |      |      |
|  6 | g     | dm   | dd   |      |      |      |      |
|  7 | g     | dm   | ee   |      |      |      |      |
+----+-------+------+------+------+------+------+------+
```

## step 7  <remove_named_policy>

```rust
e.remove_named_policy(
    "g",
    vec!["cm".to_string(), "cd".to_string()],
)
.await
.unwrap();
```


result `id=5 is removed`


```

+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
|  6 | g     | dm   | dd   |      |      |      |      |
|  7 | g     | dm   | ee   |      |      |      |      |
+----+-------+------+------+------+------+------+------+

```

## step 8  <remove_named_policies>

```rust
e.remove_named_policies(
    "g",
    vec![
        vec!["dm".to_string(), "dd".to_string()],
        vec!["dm".to_string(), "ee".to_string()],
    ],
)
.await
.unwrap();
```

log

```log
Dec 02 09:31:31.929 INFO Policy Management, Event: Type: RemovePolicies, Assertion: p::g, Removed: 2
test test::remove_named_policies ... ok
```

result `id=6 and id=7 is removed`

```
+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
+----+-------+------+------+------+------+------+------+
```

## step 9  <remove_filtered_policy>

rerun test step1 to step 4;

result:
```
+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
|  8 | p     | a    | b    | c    |      |      |      |
|  9 | p     | bm   | bd   | rw   |      |      |      |
| 10 | p     | bm   | be   | r    |      |      |      |
| 11 | g     | cm   | cd   |      |      |      |      |
| 12 | g     | dm   | dd   |      |      |      |      |
| 13 | g     | dm   | ee   |      |      |      |      |
+----+-------+------+------+------+------+------+------+
```

begin test

```rust
e.remove_filtered_policy(0, vec!["bm".to_string()])
            .await
            .unwrap();
```


log

```log
Dec 02 09:36:20.897 INFO Policy Management, Event: Type: RemoveFilteredPolicy, Assertion: p::p, Removed: 2
test test::remove_filtered_policy ... ok
```

result `id=9 and id=10 is removed because V0 = bm`

```
+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
|  8 | p     | a    | b    | c    |      |      |      |
| 11 | g     | cm   | cd   |      |      |      |      |
| 12 | g     | dm   | dd   |      |      |      |      |
| 13 | g     | dm   | ee   |      |      |      |      |
+----+-------+------+------+------+------+------+------+
```

## step 10  <remove_filtered_named_policy>

```rust
e.remove_filtered_named_policy("g", 0, vec!["dm".to_string()])
    .await
    .unwrap();
e.remove_filtered_named_policy("p", 1, vec!["b".to_string()])
    .await
    .unwrap();
```



result    
`id=12 and id=13 is removed ==> ptype = g,v0=dm`   
`id=8 is removed   => ptype = p ,v1 = b`

```
+----+-------+------+------+------+------+------+------+
| id | ptype | v0   | v1   | v2   | v3   | v4   | v5   |
+----+-------+------+------+------+------+------+------+
|  1 | p     | am   | ad   | rw   |      |      |      |
| 11 | g     | cm   | cd   |      |      |      |      |
+----+-------+------+------+------+------+------+------+
```

## step 11 <enforce>

```rust
  e.enforce(("am", "ad", "rw")).unwrap();
  e.enforce(("am", "ad", "cd")).unwrap();
```

log

```log
Dec 02 10:01:34.307 INFO Enforce Request, Response: true, Cached: false, Request: am,ad,rw
Dec 02 10:01:34.311 INFO Enforce Request, Response: false, Cached: false, Request: am,ad,cd
test test::test_enforce ... ok
```
