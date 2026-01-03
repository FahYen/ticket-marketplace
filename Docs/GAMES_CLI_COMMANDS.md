# Games Admin CLI Commands

## Prerequisites

1. Set `ADMIN_API_KEY` in your `.env` file:
   ```
   ADMIN_API_KEY=your-secret-api-key-here
   ```

2. Make sure the backend server is running (port 3000)

3. Get your API key from `.env` (replace `your-secret-api-key-here` in commands below)

---

## Add a Game

### Football Game Example
```bash
curl -X POST http://localhost:3000/api/games \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-secret-api-key-here" \
  -d '{
    "sport_type": "football",
    "name": "Richmond @ Spartan Football",
    "game_time": "2026-09-09T15:30:00Z"
  }'
```

### Basketball Game Example
```bash
curl -X POST http://localhost:3000/api/games \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-secret-api-key-here" \
  -d '{
    "sport_type": "basketball",
    "name": "Michigan @ MSU Basketball",
    "game_time": "2026-12-15T19:00:00Z"
  }'
```

### Hockey Game Example
```bash
curl -X POST http://localhost:3000/api/games \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-secret-api-key-here" \
  -d '{
    "sport_type": "hockey",
    "name": "Ohio State @ MSU Hockey",
    "game_time": "2026-11-20T18:00:00Z"
  }'
```

---

## Delete a Game

Replace `<game-id>` with the actual UUID from the create response:

```bash
curl -X DELETE http://localhost:3000/api/games/<game-id> \
  -H "Authorization: Bearer your-secret-api-key-here"
```

**Example:**
```bash
curl -X DELETE http://localhost:3000/api/games/123e4567-e89b-12d3-a456-426614174000 \
  -H "Authorization: Bearer your-secret-api-key-here"
```

---

## Using Environment Variable for API Key

You can store the API key in a variable to avoid repeating it:

```bash
# Set the API key
ADMIN_API_KEY="your-secret-api-key-here"

# Add a game
curl -X POST http://localhost:3000/api/games \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $ADMIN_API_KEY" \
  -d '{
    "sport_type": "football",
    "name": "Richmond @ Spartan Football",
    "game_time": "2026-09-09T15:30:00Z"
  }'
```

---

## Date Format

Use ISO 8601 format (UTC):
- Format: `YYYY-MM-DDTHH:MM:SSZ`
- Examples:
  - `2026-09-09T15:30:00Z` (3:30 PM UTC)
  - `2026-12-15T19:00:00Z` (7:00 PM UTC)
  - `2026-11-20T18:00:00Z` (6:00 PM UTC)

**Convert EST to UTC:**
- EST is UTC-5 (or UTC-4 during daylight saving)
- Example: 3:30 PM EST = 8:30 PM UTC (or 7:30 PM UTC during DST)
- Use: `2026-09-09T20:30:00Z` for 3:30 PM EST (assuming EDT)

---

## Expected Responses

### Success (201 Created)
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "sport_type": "Football",
  "name": "Richmond @ Spartan Football",
  "game_time": "2026-09-09T15:30:00Z",
  "cutoff_time": "2026-09-09T14:30:00Z"
}
```

### Error - Unauthorized (401)
```json
{
  "error": "Unauthorized"
}
```

### Error - Invalid Sport Type (400)
```json
{
  "error": "Invalid sport type"
}
```

### Error - Missing Fields (400)
```json
{
  "error": "Internal server error"
}
```

---

## Quick Test Script

Save this as `add-game.sh`:

```bash
#!/bin/bash

ADMIN_API_KEY="${ADMIN_API_KEY:-your-secret-api-key-here}"
SPORT_TYPE="${1:-football}"
GAME_NAME="${2:-Test Game}"
GAME_TIME="${3:-2026-09-09T15:30:00Z}"

curl -X POST http://localhost:3000/api/games \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $ADMIN_API_KEY" \
  -d "{
    \"sport_type\": \"$SPORT_TYPE\",
    \"name\": \"$GAME_NAME\",
    \"game_time\": \"$GAME_TIME\"
  }" | jq

# Usage:
# ./add-game.sh football "Michigan @ MSU" "2026-09-09T15:30:00Z"
```

Make it executable:
```bash
chmod +x add-game.sh
```

---

## Notes

- The `cutoff_time` is automatically calculated as `game_time - LISTING_CUTOFF_MINUTES`
- `LISTING_CUTOFF_MINUTES` defaults to 60 minutes (set in `.env`)
- Valid `sport_type` values: `"football"`, `"basketball"`, `"hockey"`
- Game time must be in the future
- Game name cannot be empty

