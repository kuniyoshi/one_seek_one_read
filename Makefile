MODE=archive
COUNT=100
SEQ_TYPE=random
OPTIMIZATION=true

data_dir=data
access_log=$(data_dir)/access.log
resource_list=$(data_dir)/resource.list
resource_index=$(data_dir)/resource.index
resource_dir=$(data_dir)/resource.d
archive=$(data_dir)/archive.data

list_resource:
	cat $(access_log) \
		| scripts/step1.extract.pl \
		| scripts/step2.clean_up_path.pl \
		| scripts/list_resource.pl >$(resource_list)

create_dummy:
	cat $(resource_list) \
		| scripts/create_dummy.pl -out_dir=$(resource_dir) >$(resource_index)

archive:
	cat $(resource_index) \
		| scripts/archive.pl -out=$(archive)

clean:
	rm -Rf $(resource_dir)
	cargo clean

test_data:
	scripts/test.pl

run:
	RUST_LOG=$(RUST_LOG) cargo run --bin main --verbose -- $(MODE) $(COUNT) $(SEQ_TYPE) $(OPTIMIZATION)

release:
	cargo build --release

test:
	cargo test

bench:
	cargo bench
