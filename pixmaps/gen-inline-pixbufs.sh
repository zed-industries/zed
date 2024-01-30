#! /bin/sh

prefix=stock_
list=

for file in "$@"; do
    name=$(echo "$file" | sed 's|-|_|g; s|^.*/||; s|\..*$||')
    list="$list $prefix$name $file"
done

gdk-pixbuf-csource --raw --static --build-list $list
