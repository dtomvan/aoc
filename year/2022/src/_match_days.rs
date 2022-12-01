match arg.as_str() {
	"0" => day0::main,
	"1" => day1::main,
    _ => unimplemented!(),
}
