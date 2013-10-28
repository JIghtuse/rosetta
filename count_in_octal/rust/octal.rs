fn main() {
	for i in range(std::uint::min_value, std::uint::max_value) {
		println(fmt!("%o", i));
	}
}
