use super::*;

use num_traits::Num;

#[test]
fn with_atom_is_bad_argument() {
    let mut process: Process = Default::default();
    let atom_term = Term::str_to_atom("atom", Existence::DoNotCare, &mut process).unwrap();

    assert_bad_argument!(erlang::head(atom_term), process);
}

#[test]
fn with_empty_list_is_bad_argument() {
    assert_bad_argument!(erlang::head(Term::EMPTY_LIST), Default::default());
}

#[test]
fn with_list_returns_head() {
    let mut process: Process = Default::default();
    let head_term = Term::str_to_atom("head", Existence::DoNotCare, &mut process).unwrap();
    let list_term = Term::cons(head_term, Term::EMPTY_LIST, &mut process);

    assert_eq_in_process!(erlang::head(list_term), Ok(head_term), process);
}

#[test]
fn with_small_integer_is_bad_argument() {
    let mut process: Process = Default::default();
    let small_integer_term = 0.into_process(&mut process);

    assert_bad_argument!(erlang::head(small_integer_term), process);
}

#[test]
fn with_big_integer_is_bad_argument() {
    let mut process: Process = Default::default();
    let big_integer_term = <BigInt as Num>::from_str_radix("576460752303423489", 10)
        .unwrap()
        .into_process(&mut process);

    assert_bad_argument!(erlang::head(big_integer_term), process);
}

#[test]
fn with_float_is_bad_argument() {
    let mut process: Process = Default::default();
    let float_term = 1.0.into_process(&mut process);

    assert_bad_argument!(erlang::head(float_term), process);
}

#[test]
fn with_local_pid_is_bad_argument() {
    let process: Process = Default::default();
    let local_pid_term = Term::local_pid(0, 0).unwrap();

    assert_bad_argument!(erlang::head(local_pid_term), process);
}

#[test]
fn with_external_pid_is_bad_argument() {
    let mut process: Process = Default::default();
    let external_pid_term = Term::external_pid(1, 0, 0, &mut process).unwrap();

    assert_bad_argument!(erlang::head(external_pid_term), process);
}

#[test]
fn with_tuple_is_bad_argument() {
    let mut process: Process = Default::default();
    let tuple_term = Term::slice_to_tuple(&[], &mut process);

    assert_bad_argument!(erlang::head(tuple_term), process);
}

#[test]
fn with_heap_binary_is_bad_argument() {
    let mut process: Process = Default::default();
    let heap_binary_term = Term::slice_to_binary(&[], &mut process);

    assert_bad_argument!(erlang::head(heap_binary_term), process);
}

#[test]
fn with_subbinary_is_bad_argument() {
    let mut process: Process = Default::default();
    let binary_term =
        Term::slice_to_binary(&[0b0000_00001, 0b1111_1110, 0b1010_1011], &mut process);
    let subbinary_term = Term::subbinary(binary_term, 0, 7, 2, 1, &mut process);

    assert_bad_argument!(erlang::head(subbinary_term), process);
}
