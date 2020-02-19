access_log=access.log
resource_list=resource.list
resource_index=resource.index
resource_dir=resource.d
archive=archive.data

list_resource:
	cat $(access_log) \
		| perl step1.extract.pl \
		| perl step2.clean_up_path.pl \
		| perl list_resource.pl >$(resource_list)

create_dummy:
	cat $(resource_list) \
		| perl -s create_dummy.pl -out_dir=$(resource_dir) >$(resource_index)

archive:
	cat $(resource_index) \
		| perl -s archive.pl -out=$(archive)

clean:
	rm $(archive)
	rm $(resource_index)
	rm -Rf $(resource_dir)

test_data:
	perl test.pl

run:
	cargo build
	RUST_LOG=$(RUST_LOG) cargo run --bin archive --verbose

release:
	cargo build --release

test:
	cargo test
