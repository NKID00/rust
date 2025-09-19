// Regression test for #130395 and #146261

enum A {
    B,
    C(),
    D(i32),
}

fn foo(_: A) -> impl Fn() {
    || {}
}

fn main() {
    // Suggestion could be improved to indicate letter case confusion
    foo(A::b); //~ ERROR no variant or associated item named `b` found for enum `A` in the current scope [E0599]
    foo(A::c); //~ ERROR no variant or associated item named `c` found for enum `A` in the current scope [E0599]
    foo(A::d); //~ ERROR no variant or associated item named `d` found for enum `A` in the current scope [E0599]
    (A::b)(); // FIXME: Suggestion is malformed
    //~^ ERROR no variant or associated item named `b` found for enum `A` in the current scope [E0599]
    (A::c)(); //~ ERROR no variant or associated item named `c` found for enum `A` in the current scope [E0599]
    (A::d)(); //~ ERROR no variant or associated item named `d` found for enum `A` in the current scope [E0599]
    foo(A::b)(); //~ ERROR no variant or associated item named `b` found for enum `A` in the current scope [E0599]
    foo(A::c)(); //~ ERROR no variant or associated item named `c` found for enum `A` in the current scope [E0599]
    foo(A::d)(); //~ ERROR no variant or associated item named `d` found for enum `A` in the current scope [E0599]
}
