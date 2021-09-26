# https://pandoc.org/MANUAL.html#slide-shows

SLIDE_OPTIONS=-t revealjs --css=./custom.css -s

all: rust-for-geo.html

rust-for-geo.html: rust-for-geo.md reveal.js/css/reveal.css
	pandoc $(SLIDE_OPTIONS) $< -o $@

watch: rust-for-geo.md reveal.js/css/reveal.css rust-for-geo.html
	fswatch -o --event Updated $< | xargs -n1 -I{} sh -c "echo Rebuilding...; pandoc $(SLIDE_OPTIONS) $< -o rust-for-geo.html"

full: rust-for-geo.md reveal.js/css/reveal.css
	pandoc  $(SLIDE_OPTIONS) --self-contained $< -o rust-for-geo.html

reveal_version=3.6.0

reveal.js/css/reveal.css:
	wget -O /tmp/$(reveal_version).tar.gz https://github.com/hakimel/reveal.js/archive/$(reveal_version).tar.gz
	tar xvzf /tmp/$(reveal_version).tar.gz
	rm /tmp/$(reveal_version).tar.gz
	mv reveal.js-$(reveal_version) reveal.js
	touch $@
