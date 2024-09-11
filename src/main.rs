use std::io;

fn fib(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    println!("Digite um número positivo: ");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Digite um número válido...");
    let n: i64 = n.trim().parse().expect("Por favor, digite um número...");

    let resultado = fib(n);
    println!("O termo {} da sequência de Fibonacci é: {}\n", n, resultado);
}