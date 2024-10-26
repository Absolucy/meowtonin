import re


def parse_header(header_file):
	function_pattern = re.compile(
		r"^\s*(?:inline\s+)?(?:\w+\s+)+(\w+)\s*\([^)]*\)\s*(?:;|\{)"
	)
	functions = []

	with open(header_file, "r") as file:
		for line in file:
			match = function_pattern.match(line)
			if match:
				functions.append(match.group(1))

	return functions


def generate_stub_file(functions, output_file):
	with open(output_file, "w") as file:
		for function in functions:
			file.write(f"__attribute__((weak)) void {function}(void) {{}}\n")


def main():
	header_file = "../../bindings/byondapi.h"
	output_file = "stub.c"

	functions = parse_header(header_file)
	generate_stub_file(functions, output_file)
	print(f'Stub file "{output_file}" generated with {len(functions)} functions.')


if __name__ == "__main__":
	main()
