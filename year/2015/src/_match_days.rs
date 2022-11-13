match arg.as_str() {
	"1" => day1::main,
	"2" => day2::main,
	"15" => day15::main,
    _ => unimplemented!(),
}
