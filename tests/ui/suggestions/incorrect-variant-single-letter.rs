// Regression test for #146261

enum A {
    B(),
}

fn foo(_: A) {}

fn main() {
    foo(A::C); //~ ERROR no variant or associated item named `C` found for enum `A` in the current scope [E0599]
}
