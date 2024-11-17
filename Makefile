ZTM_POZNAN_HTML=ztm.poznan.html
ZTM_POZNAN=$(addprefix ztm.poznan.pl/,$(shell grep -oE '2[0-9]{7}_2[0-9]{7}.zip' $(ZTM_POZNAN_HTML)))

show: mkdata
	@sort ztm.poznan.pl/*.tsv | sort -rn | uniq

trams: mkdata
	@grep --no-filename 'Tramway' ztm.poznan.pl/*.tsv | sort -rn | uniq

update:
	curl -L 'https://www.ztm.poznan.pl/otwarte-dane/gtfsfiles/' \
		-H "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0" \
		-H "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8" \
		-H "Accept-Language: pl,en-US;q=0.7,en;q=0.3" \
		-H "Accept-Encoding: gzip, deflate, br, zstd" \
		-H "Connection: keep-alive" \
		-H "Upgrade-Insecure-Requests: 1" \
		-H "Sec-Fetch-Dest: document" \
		-H "Sec-Fetch-Mode: navigate" \
		-H "Sec-Fetch-Site: cross-site" \
		-H "Priority: u=0, i" \
		| zcat >$(ZTM_POZNAN_HTML)

mkdata: $(addsuffix .tsv,$(ZTM_POZNAN)) $(ZTM_POZNAN)

ztm.poznan.pl/%.zip.tsv: ztm.poznan.pl/%.zip
	cargo run --quiet --release -- $< > $@

ztm.poznan.pl/%.zip:
	mkdir -p $(dir $@)
	curl -s "https://www.ztm.poznan.pl/pl/dla-deweloperow/getGTFSFile/?file=$(notdir $@)" -o "$@"

.PHONY: sorted mkdata trams update
