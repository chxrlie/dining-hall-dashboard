## 📡 API Documentation

The application provides a comprehensive RESTful API for programmatic access to all features.

### Authentication Endpoints

| Method | Endpoint        | Description        |
| ------ | --------------- | ------------------ |
| `GET`  | `/admin/login`  | Display login page |
| `POST` | `/admin/login`  | Authenticate user  |
| `POST` | `/admin/logout` | End user session   |

### Menu Item Endpoints

| Method   | Endpoint          | Description          |
| -------- | ----------------- | -------------------- |
| `GET`    | `/api/items`      | List all menu items  |
| `POST`   | `/api/items`      | Create new menu item |
| `PUT`    | `/api/items/{id}` | Update menu item     |
| `DELETE` | `/api/items/{id}` | Delete menu item     |
| `POST`   | `/api/items/reload` | Reload menu items  |

### Notice Endpoints

| Method   | Endpoint            | Description       |
| -------- | ------------------- | ----------------- |
| `GET`    | `/api/notices`      | List all notices  |
| `POST`   | `/api/notices`      | Create new notice |
| `PUT`    | `/api/notices/{id}` | Update notice     |
| `DELETE` | `/api/notices/{id}` | Delete notice     |
| `POST`   | `/api/notices/reload` | Reload notices    |

### Menu Preset Endpoints

| Method   | Endpoint            | Description              |
| -------- | ------------------- | ------------------------ |
| `GET`    | `/api/presets`      | List all menu presets    |
| `POST`   | `/api/presets`      | Create new menu preset   |
| `GET`    | `/api/presets/{id}` | Get specific menu preset |
| `PUT`    | `/api/presets/{id}` | Update menu preset       |
| `DELETE` | `/api/presets/{id}` | Delete menu preset       |
| `POST`   | `/api/presets/reload` | Reload menu presets    |

### Schedule Endpoints

| Method   | Endpoint                  | Description                  |
| -------- | ------------------------- | ---------------------------- |
| `GET`    | `/api/schedules`          | List all menu schedules      |
| `POST`   | `/api/schedules`          | Create new menu schedule     |
| `GET`    | `/api/schedules/{id}`     | Get specific menu schedule   |
| `PUT`    | `/api/schedules/{id}`     | Update menu schedule         |
| `DELETE` | `/api/schedules/{id}`     | Delete menu schedule         |
| `GET`    | `/api/schedules/upcoming` | List upcoming schedules      |
| `POST`   | `/api/schedules/validate` | Validate schedule parameters |
| `POST`   | `/api/schedules/reload`   | Reload menu schedules      |

### API Response Examples

#### Error Handling

The API provides standardized error responses in a consistent JSON format. If a request fails, the response will include an `error` and `error_type` field.

**Example: 404 Not Found**

```http
GET /api/items/invalid-id
```

Response:

```json
{
  "error": "Not Found",
  "message": "Menu item with id invalid-id not found",
  "error_type": "NOT_FOUND"
}
```