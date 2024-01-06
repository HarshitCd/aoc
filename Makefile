DAY := day2
make run:
	@cd $(DAY) && cargo build
	@cd $(DAY) && cargo run
