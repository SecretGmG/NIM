# Game Architecture Summary

## Overview

A two-player abstract strategy game implemented in **Godot**. The board consists of **nodes (points)** connected by **lines**. Each turn, a player selects a line and activates one or more points on it. Includes:

* Game mode (with AI in Rust)
* Editor mode (for level creation)
* Board logic decoupled from game/editor logic
* Rendering separated from logic via MVC-like design

---

## Data Model

### Node Types

* All nodes are structurally identical
* Mode determines if they behave as control points or game points

### `NodeState` Enum

```gdscript
enum NodeState {
    Default,
    Activated,
    ControlVisible,
    ControlHidden
}
```

### `VisualState` Struct

```gdscript
class_name VisualState
extends RefCounted

var hovered: bool
var selected: bool
var selectable: bool
```

---

## Core Components

### PointController

```gdscript
class_name PointController
extends Node2D

var id: int
var node_state: NodeState

signal mouse_entered(id: int)
signal mouse_exited(id: int)
signal clicked(id: int)

func set_node_state(state: NodeState) -> void
func set_visual_state(visual: VisualState) -> void
```

### PointView

```gdscript
class_name PointView
extends Node2D

func update_visuals(state: NodeState, visual: VisualState) -> void
```

---

### LineController

```gdscript
class_name LineController
extends Node

var id: int
var node_state: NodeState
var point_ids: Array[int]  # Ordered list of node IDs

signal mouse_entered(id: int)
signal mouse_exited(id: int)
signal clicked(id: int)

func set_node_state(state: NodeState) -> void
func set_visual_state(visual: VisualState) -> void
func get_point_or_control_point(index: int) -> int
```

### LineView

```gdscript
class_name LineView
extends Node2D

func update_visuals(state: NodeState, visual: VisualState) -> void
```

---

### BoardController

```gdscript
class_name BoardController
extends Node

signal point_hovered(id: int)
signal point_unhovered(id: int)
signal point_clicked(id: int)

signal line_hovered(id: int)
signal line_unhovered(id: int)
signal line_clicked(id: int)

func add_point(id: int, pos: Vector2) -> void
func remove_point(id: int) -> void
func move_point(id: int, new_pos: Vector2) -> void

func add_line(id: int, point_ids: Array[int]) -> void
func remove_line(id: int) -> void
func insert_point_in_line(line_id: int, idx: int, point_id: int) -> void

func get_point_position(id: int) -> Vector2
func get_point_line_ids(id: int) -> Array[int]
func get_line_point_ids(line_id: int) -> Array[int]

func set_node_state(id: int, state: NodeState) -> void
func set_visual_state(id: int, visual: VisualState) -> void
```

---

## Interaction Flow

### Points and Lines

* Mouse events are emitted from view to controller
* Controller emits typed signals with IDs
* BoardController receives and forwards these to game/editor
* Game or Editor updates visual and logical state explicitly

---

## Editor Features

* Add/move/delete points
* Add/delete lines
* Add/remove points to/from lines
* Change node state (activated, control visible/hidden)
* Drag control points to change line shape
* All points are editable; "control points" are just in a different visual state
* Control over visibility of control points is driven by EditorController

---

## Game Features

* AI in Rust communicates via FFI or WebAssembly
* GameController handles:

  * Turn logic
  * Line selection and point activation rules
  * Hover highlights and hints
  * Delegates all visual and input logic to BoardController