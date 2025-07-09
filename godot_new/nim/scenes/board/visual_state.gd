# Data structure for passing visual state info to update visuals
class_name VisualInfo

var selected: bool = false
var focused: bool = false 
# like hovered, but diffrent name for clarity, since the mouse can hover over multiple things at once
# but not all of them need to be in focus necessarily
var possible_to_select: bool = false
