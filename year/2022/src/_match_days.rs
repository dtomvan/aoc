match arg.as_str() {
	"1" => day1::main,
	"2" => day2::main,
    _ => unimplemented!(),
}
