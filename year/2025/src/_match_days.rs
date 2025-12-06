match arg.as_str() {
	"2" => day2::main,
	"5" => day5::main,
    _ => unimplemented!(),
}
