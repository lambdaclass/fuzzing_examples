docker-up:
	docker build -t ready-to-fuzz-image .
	docker run -it --rm --cap-add=SYS_PTRACE --security-opt seccomp=unconfined ready-to-fuzz-image bash

# Step on the folder with the fuzzer and run with the corresponding target
LIBFUZZER = fuzz_target_1
run-libfuzzer:
	cd fuzz
	cargo +nightly fuzz run $(LIBFUZZER)

# Step on the folder with the fuzzer and run with the corresponding fuzzer name
HONGGFUZZER = fuzz
run-honggfuzzer:
	cargo hfuzz run $(HONGGFUZZER)
