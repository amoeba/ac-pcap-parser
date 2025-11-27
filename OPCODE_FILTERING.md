# Opcode Filtering Implementation

## Overview

Users can now filter messages by opcode and direction in three interfaces:
- **CLI**: Command-line with `--filter-opcode` argument
- **TUI**: Terminal UI with `o` (opcode) and `f` (direction) keys  
- **Web/Desktop GUI**: Interactive UI with opcode and direction dropdowns/inputs

The same opcode value is findable by any valid input format (e.g., `0xF7B1` and `63409` both find the same messages).

## CLI Usage

### Messages Command with Opcode Filter

```bash
# Filter by hex opcode (with 0x prefix)
cargo run -- messages --filter-opcode 0xF7B1

# Filter by hex opcode (lowercase)
cargo run -- messages --filter-opcode 0xf7b1

# Filter by decimal opcode
cargo run -- messages --filter-opcode 63409

# Combine with other filters
cargo run -- messages --filter-type "Item" --filter-opcode 0xF7B1 --direction Send
cargo run -- messages -t "Item" -o 0xF7B1 -d Send  # Short form
```

### Supported Formats

- `0xF7B1` - Hex with 0x prefix (uppercase)
- `0xf7b1` - Hex with 0x prefix (lowercase)
- `0xF7b1` - Hex with 0x prefix (mixed case)
- `63409` - Decimal format
- Invalid: `F7B1` without prefix is interpreted as decimal, not hex

## TUI Usage

### Interactive Key Bindings

| Key | Action |
|-----|--------|
| `/` | Search by message type (substring match) |
| `o` | Filter by opcode (enter hex/decimal value) |
| `f` | Cycle through direction filters (All → Send → Recv → All) |
| `Enter` | Apply type or opcode filter (when in search mode) |
| `Esc` | Clear all filters |

### TUI Filter Example

1. Press `o` to enter opcode filter mode
2. Type `0xF7B1` or `63409` (both work)
3. Press `Enter` to apply
4. Status bar shows: `o:OpCode(F7B1)`

Press `f` to toggle direction filtering:
- Status shows: `f:Dir(All)` → `f:Dir(Send)` → `f:Dir(Recv)` → back to `f:Dir(All)`

## Web/Desktop GUI Usage

### Desktop Layout
- **OpCode field**: Enter hex (`0xF7B1`) or decimal (`63409`)
- **Direction dropdown**: Select All, Send, or Recv
- Both filters update live as you type/select
- Works with the Search field for combined filtering

### Mobile Layout
- **OpCode field**: Compact entry (70px width, shows "OpCode" hint)
- **Direction dropdown**: Shows "S" for Send, "R" for Recv, "All" for all directions
- Third row dedicated to filters to keep mobile UI compact

### Web/Desktop Filter Example

1. Type `0xF7B1` in the OpCode field
2. Select "Send" from the Direction dropdown
3. Results automatically filter to show only Send messages with opcode 0xF7B1
4. Can combine with Search field (e.g., search for "Item" while filtering by opcode)

## Implementation Details

### New Module: `crates/cli/src/filter.rs`

Two public functions:

```rust
/// Parse an opcode filter string (hex or decimal) to u32
pub fn parse_opcode_filter(s: &str) -> Result<u32>

/// Convert opcode hex string (e.g., "F7B1") to u32
pub fn opcode_str_to_u32(opcode_str: &str) -> Option<u32>
```

### CLI Changes (`crates/cli/src/main.rs`)

- Added `--filter-opcode` (`-o`) argument to Messages command
- Updated `output_messages()` function to parse and apply opcode filter
- Filter logic: if opcode matches, include message in results

### TUI Changes (`crates/cli/src/tui.rs`)

- Added `filter_opcode: Option<u32>` field to `App` struct
- Added `filter_direction: Option<String>` field to `App` struct
- Enhanced `filtered_messages()` to apply both opcode and direction filters
- Added 'f' key handler to cycle through direction filters
- Added 'o' key handler to enter opcode filter mode
- Updated search/filter logic to parse opcode strings when Enter is pressed
- Enhanced help text to show current filter status

### Web/Desktop GUI Changes (`crates/web/src/`)

#### New Module: `crates/web/src/filter.rs`
- Identical to CLI filter module for consistency
- `parse_opcode_filter()`: Parse hex or decimal opcode
- `opcode_str_to_u32()`: Convert hex opcode string to u32

#### Main App (`crates/web/src/lib.rs`)
- Added `filter_opcode: Option<u32>` field
- Added `filter_direction: Option<String>` field
- Desktop layout: OpCode input field + Direction dropdown after Search
- Mobile layout: Third row with compact OpCode field + Direction dropdown
- Live filtering as user types in OpCode field

#### Packet List UI (`crates/web/src/ui/packet_list.rs`)
- Updated `show_messages_list()` to apply opcode and direction filters
- Filters combined with search and time scrubber filters
- Display count shows filtered vs total messages

## Test Coverage

12 comprehensive tests in `filter.rs`:

### Basic Format Parsing (3 tests)
- Hex with uppercase prefix
- Hex with lowercase prefix
- Hex with mixed case prefix

### Format Variations (2 tests)
- Decimal format
- Invalid inputs (hex and decimal)

### Utility Function (1 test)
- `opcode_str_to_u32()` conversion

### Equivalence Tests (6 tests)
- **Hex vs Decimal equivalence**: Verifies that `0xF7B1` and `63409` produce identical u32 values
- **Multiple opcodes**: Tests equivalence for different opcode values (0x02CD, 0x0000, 0x0001, 0xFFFF)
- **Message matching**: Verifies that filter values match message opcode strings
- **End-to-end filtering**: Ensures both input formats would match the same messages

### Key Test: `test_decimal_and_hex_produce_identical_filtering_value()`

This test simulates the actual filtering flow:
1. Get opcode string from message (`"F7B1"`)
2. Convert to u32 (`opcode_str_to_u32("F7B1")`)
3. Parse hex filter (`parse_opcode_filter("0xF7B1")`)
4. Parse decimal filter (`parse_opcode_filter("63409")`)
5. Assert all three produce the same value

This guarantees users can enter either format and find the same messages.

## Usage Examples

### Find all 0xF7B1 messages
```bash
# CLI hex
cargo run -- messages -o 0xF7B1

# CLI decimal
cargo run -- messages -o 63409

# TUI: press 'o', type '0xF7B1' or '63409', press Enter
```

### Find Send-only 0x02CD messages
```bash
cargo run -- messages -o 0x02CD -d Send
# or
cargo run -- messages -o 717 -d Send
```

### Use TUI interactively
```bash
cargo run -- tui
# Then press 'f' to filter to Send messages, 'o' and type '0xF7B1'
```
