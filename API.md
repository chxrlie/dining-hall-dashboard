# Dining Hall Dashboard API Documentation

## 📋 Overview

The Dining Hall Dashboard provides a comprehensive RESTful API for programmatic access to all application features. This document details all available endpoints, request/response formats, and error codes.

## 🔐 Authentication

Most API endpoints require authentication via session cookies. To authenticate:

1. Make a POST request to `/admin/login` with username and password
2. The server will set a session cookie in the response
3. Include this cookie in subsequent requests to protected endpoints

### Login

```http
POST /admin/login
Content-Type: application/json

{
  "username": "admin",
  "password": "admin123"
}
```

Response:

```http
HTTP/1.1 303 See Other
Location: /admin
Set-Cookie: session=...
```

### Logout

```http
POST /admin/logout
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "error": "Logout successful"
}
```

## 🍽️ Menu Items API

### Get All Menu Items

```http
GET /api/items
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "11111111-1111-1111-1111-111111111111",
    "name": "Grilled Chicken Breast",
    "category": "Mains",
    "description": "Tender grilled chicken breast with herbs and spices",
    "allergens": ["Soy"],
    "is_available": true
  }
]
```

### Create Menu Item

```http
POST /api/items
Content-Type: application/json

{
  "name": "Vegetable Stir Fry",
  "category": "Mains",
  "description": "Fresh seasonal vegetables stir-fried with tofu",
  "allergens": ["Soy", "Gluten"],
  "is_available": true
}
```

Response:

```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "id": "22222222-2222-2222-2222-222222222222",
  "name": "Vegetable Stir Fry",
  "category": "Mains",
  "description": "Fresh seasonal vegetables stir-fried with tofu",
  "allergens": ["Soy", "Gluten"],
  "is_available": true
}
```

### Update Menu Item

```http
PUT /api/items/{id}
Content-Type: application/json

{
  "name": "Vegetable Stir Fry (Updated)",
  "category": "Mains",
  "description": "Fresh seasonal vegetables stir-fried with tofu and sesame seeds",
  "allergens": ["Soy", "Gluten", "Sesame"],
  "is_available": true
}
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "22222222-2222-2222-2222-222222222222",
  "name": "Vegetable Stir Fry (Updated)",
  "category": "Mains",
  "description": "Fresh seasonal vegetables stir-fried with tofu and sesame seeds",
  "allergens": ["Soy", "Gluten", "Sesame"],
  "is_available": true
}
```

### Delete Menu Item

```http
DELETE /api/items/{id}
```

Response:

```http
HTTP/1.1 204 No Content
```

## 📢 Notices API

### Get All Notices

```http
GET /api/notices
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "33333333-3333-3333-3333-333333333333",
    "title": "Special Holiday Hours",
    "content": "The dining hall will be closed on December 25th for Christmas.",
    "is_active": true,
    "created_at": "2023-12-01T10:00:00Z",
    "updated_at": "2023-12-01T10:00:00Z"
  }
]
```

### Create Notice

```http
POST /api/notices
Content-Type: application/json

{
  "title": "New Menu Items Available",
  "content": "We've added several new vegetarian options to our menu this week!",
  "is_active": true
}
```

Response:

```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "id": "44444444-4444-4444-4444-444444444444",
  "title": "New Menu Items Available",
  "content": "We've added several new vegetarian options to our menu this week!",
  "is_active": true,
  "created_at": "2023-12-10T14:30:00Z",
  "updated_at": "2023-12-10T14:30:00Z"
}
```

### Update Notice

```http
PUT /api/notices/{id}
Content-Type: application/json

{
  "title": "Updated Menu Items Available",
  "content": "We've added several new vegetarian and vegan options to our menu this week!",
  "is_active": true
}
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "44444444-4444-4444-4444-444444444444",
  "title": "Updated Menu Items Available",
  "content": "We've added several new vegetarian and vegan options to our menu this week!",
  "is_active": true,
  "created_at": "2023-12-10T14:30:00Z",
  "updated_at": "2023-12-11T09:15:00Z"
}
```

### Delete Notice

```http
DELETE /api/notices/{id}
```

Response:

```http
HTTP/1.1 204 No Content
```

## 📋 Menu Presets API

### Get All Menu Presets

```http
GET /api/presets
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "55555555-5555-5555-5555-555555555555",
    "name": "Weekday Lunch Special",
    "description": "Standard weekday lunch menu",
    "menu_item_ids": [
      "11111111-1111-1111-1111-111111111111",
      "44444444-4444-4444-4444-444444444444",
      "77777777-7777-7777-7777-777777777777"
    ],
    "created_at": "2023-12-01T08:00:00Z",
    "updated_at": "2023-12-01T08:00:00Z"
  }
]
```

### Get Specific Menu Preset

```http
GET /api/presets/{id}
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "55555555-5555-5555-5555-555555555555",
  "name": "Weekday Lunch Special",
  "description": "Standard weekday lunch menu",
  "menu_item_ids": [
    "11111111-1111-1111-1111-111111111111",
    "44444444-4444-4444-4444-444444444444",
    "77777777-7777-7777-7777-777777777777"
  ],
  "created_at": "2023-12-01T08:00:00Z",
  "updated_at": "2023-12-01T08:00:00Z"
}
```

### Create Menu Preset

```http
POST /api/presets
Content-Type: application/json

{
  "name": "Weekend Brunch Special",
  "description": "Special weekend brunch menu",
  "menu_item_ids": [
    "22222222-2222-2222-2222-222222222222",
    "66666666-6666-6666-6666-666666666666",
    "99999999-9999-9999-9999-999999999999"
  ]
}
```

Response:

```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "id": "66666666-6666-6666-6666-666666666666",
  "name": "Weekend Brunch Special",
  "description": "Special weekend brunch menu",
  "menu_item_ids": [
    "22222222-2222-2222-2222-222222222222",
    "66666666-6666-6666-6666-666666666666",
    "99999999-9999-9999-9999-999999999999"
  ],
  "created_at": "2023-12-10T10:00:00Z",
  "updated_at": "2023-12-10T10:00:00Z"
}
```

### Update Menu Preset

```http
PUT /api/presets/{id}
Content-Type: application/json

{
  "name": "Weekend Brunch Special - Updated",
  "description": "Special weekend brunch menu with seasonal items",
  "menu_item_ids": [
    "22222222-2222-2222-2222-222222222222",
    "66666666-6666-6666-6666-666666666666",
    "88888888-8888-8888-8888-888888888888"
  ]
}
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "66666666-6666-6666-6666-666666666666",
  "name": "Weekend Brunch Special - Updated",
  "description": "Special weekend brunch menu with seasonal items",
  "menu_item_ids": [
    "22222222-2222-2222-2222-222222222222",
    "66666666-6666-6666-6666-666666666666",
    "88888888-8888-8888-8888-888888888888"
  ],
  "created_at": "2023-12-10T10:00:00Z",
  "updated_at": "2023-12-11T11:30:00Z"
}
```

### Delete Menu Preset

```http
DELETE /api/presets/{id}
```

Response:

```http
HTTP/1.1 204 No Content
```

## ⏰ Menu Schedules API

### Get All Menu Schedules

```http
GET /api/schedules
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "77777777-7777-7777-7777-777777777777",
    "preset_id": "55555555-5555-5555-5555-555555555555",
    "name": "Monday Lunch Schedule",
    "description": "Schedule for Monday lunch service",
    "start_time": "2023-12-11T11:00:00Z",
    "end_time": "2023-12-11T14:00:00Z",
    "recurrence": "Weekly",
    "status": "Active",
    "created_at": "2023-12-01T09:00:00Z",
    "updated_at": "2023-12-01T09:00:00Z"
  }
]
```

### Get Specific Menu Schedule

```http
GET /api/schedules/{id}
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "77777777-7777-7777-7777-777777777777",
  "preset_id": "55555555-5555-5555-5555-555555555555",
  "name": "Monday Lunch Schedule",
  "description": "Schedule for Monday lunch service",
  "start_time": "2023-12-11T11:00:00Z",
  "end_time": "2023-12-11T14:00:00Z",
  "recurrence": "Weekly",
  "status": "Active",
  "created_at": "2023-12-01T09:00:00Z",
  "updated_at": "2023-12-01T09:00:00Z"
}
```

### Get Upcoming Schedules

```http
GET /api/schedules/upcoming
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

[
  {
    "id": "88888888-8888-8888-8888-888888888888",
    "preset_id": "66666666-6666-6666-6666-666666666666",
    "name": "Weekend Brunch Schedule",
    "description": "Schedule for weekend brunch service",
    "start_time": "2023-12-16T09:00:00Z",
    "end_time": "2023-12-16T13:00:00Z",
    "recurrence": "Weekly",
    "status": "Pending",
    "created_at": "2023-12-10T12:00:00Z",
    "updated_at": "2023-12-10T12:00:00Z"
  }
]
```

### Create Menu Schedule

```http
POST /api/schedules
Content-Type: application/json

{
  "preset_id": "66666666-6666-6666-6666-666666666666",
  "name": "Holiday Special Schedule",
  "description": "Special schedule for holiday menu",
  "start_time": "2023-12-25T12:00:00Z",
  "end_time": "2023-12-25T16:00:00Z",
  "recurrence": "Custom",
  "status": "Pending"
}
```

Response:

```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "id": "99999999-9999-9999-9999-999999999999",
  "preset_id": "66666666-6666-6666-6666-666666666666",
  "name": "Holiday Special Schedule",
  "description": "Special schedule for holiday menu",
  "start_time": "2023-12-25T12:00:00Z",
  "end_time": "2023-12-25T16:00:00Z",
  "recurrence": "Custom",
  "status": "Pending",
  "created_at": "2023-12-11T14:00:00Z",
  "updated_at": "2023-12-11T14:00:00Z"
}
```

### Update Menu Schedule

```http
PUT /api/schedules/{id}
Content-Type: application/json

{
  "preset_id": "66666666-6666-6666-6666-666666666666",
  "name": "Holiday Special Schedule - Updated",
  "description": "Special schedule for holiday menu with extended hours",
  "start_time": "2023-12-25T11:00:00Z",
  "end_time": "2023-12-25T17:00:00Z",
  "recurrence": "Custom",
  "status": "Pending"
}
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": "99999999-9999-9999-9999-999999999999",
  "preset_id": "66666666-6666-6666-6666-666666666666",
  "name": "Holiday Special Schedule - Updated",
  "description": "Special schedule for holiday menu with extended hours",
  "start_time": "2023-12-25T11:00:00Z",
  "end_time": "2023-12-25T17:00:00Z",
  "recurrence": "Custom",
  "status": "Pending",
  "created_at": "2023-12-11T14:00:00Z",
  "updated_at": "2023-12-11T15:30:00Z"
}
```

### Delete Menu Schedule

```http
DELETE /api/schedules/{id}
```

Response:

```http
HTTP/1.1 204 No Content
```

### Validate Schedule

```http
POST /api/schedules/validate
Content-Type: application/json

{
  "preset_id": "66666666-6666-6666-6666-666666666666",
  "name": "Test Schedule",
  "description": "Schedule for testing",
  "start_time": "2023-12-20T12:00:00Z",
  "end_time": "2023-12-20T14:00:00Z",
  "recurrence": "Daily",
  "status": "Pending"
}
```

Response:

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "is_valid": true,
  "conflicts": [],
  "message": null
}
```

## 📡 Error Codes

The API uses standard HTTP status codes to indicate the success or failure of requests:

| Status Code | Description                                           |
| ----------- | ----------------------------------------------------- |
| 200         | OK - Request successful                               |
| 201         | Created - Resource created successfully               |
| 204         | No Content - Request successful, no content to return |
| 400         | Bad Request - Invalid request data                    |
| 401         | Unauthorized - Authentication required                |
| 403         | Forbidden - Access denied                             |
| 404         | Not Found - Resource not found                        |
| 409         | Conflict - Request conflicts with existing data       |
| 500         | Internal Server Error - Unexpected server error       |

### Error Response Format

All error responses follow this format:

```json
{
  "error": "Descriptive error message"
}
```

### Common Error Scenarios

#### Invalid Input

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
  "error": "Invalid category"
}
```

#### Authentication Required

```http
HTTP/1.1 401 Unauthorized
Content-Type: application/json

{
  "error": "Authentication required: Invalid username or password"
}
```

#### Resource Not Found

```http
HTTP/1.1 404 Not Found
Content-Type: application/json

{
  "error": "Menu item with id 11111111-1111-1111-1111-111111111111 not found"
}
```

## 🛠️ Rate Limiting

The API implements rate limiting to prevent abuse:

- **Authentication endpoints**: 10 requests per minute per IP
- **All other endpoints**: 100 requests per minute per authenticated session

Exceeding these limits will result in a 429 (Too Many Requests) response.

## 📱 Client Integration

### JavaScript Example

```javascript
// Login
fetch("/admin/login", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    username: "admin",
    password: "admin123",
  }),
  credentials: "include",
}).then((response) => {
  if (response.ok) {
    console.log("Login successful");
  } else {
    console.error("Login failed");
  }
});

// Get menu items
fetch("/api/items", {
  method: "GET",
  credentials: "include",
})
  .then((response) => response.json())
  .then((data) => {
    console.log("Menu items:", data);
  });
```

### Python Example

```python
import requests

# Create a session to maintain cookies
session = requests.Session()

# Login
login_data = {
    'username': 'admin',
    'password': 'admin123'
}
response = session.post('http://localhost:8080/admin/login', json=login_data)
if response.status_code == 200:
    print('Login successful')

# Get menu items
response = session.get('http://localhost:8080/api/items')
if response.status_code == 200:
    menu_items = response.json()
    print('Menu items:', menu_items)
```

## 🔒 Security Considerations

- All API endpoints use HTTPS in production
- Session cookies are marked as HttpOnly and Secure
- CSRF protection is implemented for state-changing operations
- Input validation is performed on all requests
- Passwords are hashed using Argon2 before storage
