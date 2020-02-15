access_log=access.log
resource_list=resource.list
resource_dir=resource.d
archive=archive.data

list_resource:
	cat $(access_log) \
		| perl step1.extract.pl \
		| perl step2.clean_up_path.pl \
		| perl list_resource.pl >$(resource_list)

create_dummy:
	cat $(resource_list) \
		| perl -s create_dummy.pl -out_dir=$(resource_dir)

archive:
	cat $(resource_list) \
		| perl -s archive.pl -in=$(resource_dir) -out=$(archive)

clean:
	rm -Rf $(resource_dir)

run:
	cargo run
