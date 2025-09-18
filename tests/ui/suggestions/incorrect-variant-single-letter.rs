// Regression test for #146261

enum A {
    B(),
    C(i32),
    D(isize, usize),
}

fn foo(_: A) {}

fn main() {
    // Suggestion could be improved to indicate letter case confusion
    foo(A::b); //~ ERROR no variant or associated item named `b` found for enum `A` in the current scope [E0599]
    foo(A::c); //~ ERROR no variant or associated item named `c` found for enum `A` in the current scope [E0599]
    foo(A::d); //~ ERROR no variant or associated item named `d` found for enum `A` in the current scope [E0599]
}
