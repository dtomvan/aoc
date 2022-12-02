match arg.as_str() {
	"0" => day0::main,
	"1" => day1::main,
	"2" => day2::main,
    _ => unimplemented!(),
}
