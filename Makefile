ZTM_POZNAN_HTML=ztm.poznan.html
ZTM_POZNAN=$(addprefix ztm.poznan.pl/,$(shell grep -oE '2[0-9]{7}_2[0-9]{7}.zip' $(ZTM_POZNAN_HTML)))

show: mkdata
	@sort ztm.poznan.pl/*.tsv | sort -rn | uniq

trams: mkdata
	@grep --no-filename 'Tramway' ztm.poznan.pl/*.tsv | sort -rn | uniq

update:
	curl 'https://www.ztm.poznan.pl/pl/dla-deweloperow/gtfsFiles' > $(ZTM_POZNAN_HTML)

mkdata: $(addsuffix .tsv,$(ZTM_POZNAN))

ztm.poznan.pl/%.zip.tsv: ztm.poznan.pl/%.zip
	cargo run --quiet --release -- $< > $@

ztm.poznan.pl/%.zip:
	mkdir -p $(dir $@)
	curl -s "https://www.ztm.poznan.pl/pl/dla-deweloperow/getGTFSFile/?file=$(notdir $@)" -o "$@"

.PHONY: sorted mkdata trams update
