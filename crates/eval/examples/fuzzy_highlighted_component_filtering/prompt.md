Ok, let's implment filtering in component preview. I've left a dumb `filter_editor` in component preview that we will use to drive the filtering behavior.

For this initial step, let's filter the sidebar items based on the input in the filter_editor.

You should review the code for Picker/PickerDelegate as an example, and any other that seems relevant. Perhaps uniform_list will also be good to review.

Ensure we explicitly rebuild the list/give the uniform list a new iten_count.
