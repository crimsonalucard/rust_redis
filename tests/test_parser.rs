

#[test]
fn test_resp_string() {
    for token in parse_resp("*5\r\n$0\r\n\r\n:1234\r\n+OK\r\n-wrong hello world\r\n*2\r\n+HOLA\r\n+OK\r\n").unwrap() {
        println!("{}", token);
    }
}