/*
Internal link topbar offest adjust Javascript
Code provided by @makshh on GitHub

Bug report on material-mkdocs
  https://github.com/squidfunk/mkdocs-material/issues/791
*/

// Offset top helper
function offsetY(elem) {
    if(!elem) elem = this;
    var y = elem.offsetTop;
    while (elem = elem.offsetParent) {
        y += elem.offsetTop;
    }
    return y;
}

// If a link on the same page is clicked, calculate the
// correct offset and scroll to that part of the page.
//
var links = document.getElementsByTagName('a');
for(var i = 0; i < links.length; i++) {
    links[i].onclick = function (event) {
        if (this.pathname == window.location.pathname &&
            this.protocol == window.location.protocol &&
            this.host == window.location.host) {
                event.preventDefault();
                if(this.hash.substr(1)){
                    var o = document.getElementById(this.hash.substr(1));
                    var sT = offsetY(o) - document.getElementsByClassName('md-header')[0].clientHeight;
                    window.location.hash = this.hash;
                    window.scrollTo(0, sT);
                }
        }
    }
}

// Slugify supplied text
function slugify(text){
    text = text.toLowerCase();
    text = text.replace(" ", "-");
    return text;
}

// If there is a hash in the url, slugify it
// and replace
if(window.location.hash) {
    // Fragment exists
    slug = slugify(window.location.hash);
    history.replaceState(undefined, undefined, slug)
    //window.location.hash = slug;
    document.location.replace(window.location.href);
}
