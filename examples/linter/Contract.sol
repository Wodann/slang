contract Foo {
  function f() public {
    uint aRes = tx.origin; // Failure

    uint allow = someStruct.tx.origin; // OK
  }
}
