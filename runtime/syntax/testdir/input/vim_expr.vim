" String

echo 'It''s a string'
echo 'tab: \t, new line: \n, backslash: \\'
echo "tab: \t, new line: \n, backslash: \\"

" String interpolation

echo 'Don''t highlight interpolation: {{ {1 + 2} }}'
echo "Don't highlight interpolation: {{ {1 + 2} }}"
echo $'Highlight interpolation:\t{{ { string({'foo': 'bar'}) } }}'
echo $'Highlight interpolation:\t{{ { $'nested: {{ {1 + 2} }}' } }}'
echo $"Highlight interpolation:\t{{ { string({"foo": "bar"}) } }}"
echo $"Highlight interpolation:\t{{ { $"nested: {{ {1 + 2} }}" } }}"
