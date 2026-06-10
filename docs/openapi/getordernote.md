> ## Documentation Index
> Fetch the complete documentation index at: https://developers.edesk.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Read Order Note

Return details of a order note

# OpenAPI definition

```json
{
  "openapi": "3.0.0",
  "info": {
    "title": "eDesk Open API",
    "version": "2.0",
    "description": "The eDesk OpenAPI allows you to interact with eDesk programmatically.\n\nThe below table defines the validation error codes that this API may return.\n\n| Error Code      | Description                                        |\n| --------------- | -------------------------------------------------- |\n| 4001            | Missing required field                             |\n| 4002            | Not able to find an object                         |\n| 4003            | Must be unique value                               |\n| 4004            | User can not access an object                      |\n| 4005            | Must be a numeric value                            |\n| 4006            | Must be an array                                   |\n| 4007            | Must be one of the allowed values                  |\n| 4008            | Must be a string                                   |\n| 4009            | Must be a boolean                                  |\n| 4010            | Must be a valid date                               |\n| 4011            | Must be a valid file url                           |\n| 4012            | Must be a valid image url                          |\n| 4013            | Related channel type is not supported              |\n| 4014            | Mismatch between sales order and channel           |\n| 4015            | Mismatch between client and channels               |\n| 4016            | Templates limit exceeded                           |\n| 4017            | Mismatch between sales order and sales order item  |\n| 4018            | Custom field value must match the type             |\n| 4019            | Each attachment must match format AttachmentRequest|\n| 4020            | Must be less or equal characters                   |\n| 4021            | Must be an email                                   |\n| 4022            | Shouldn't reach the message items limit            |\n"
  },
  "servers": [
    {
      "url": "https://api.edesk.com/v1",
      "description": "Main (production) endpoint"
    }
  ],
  "security": [
    {
      "bearerAuth": []
    }
  ],
  "paths": {
    "/order-notes/{orderNoteId}": {
      "get": {
        "tags": [
          "Order Notes"
        ],
        "summary": "Read Order Note",
        "description": "Return details of a order note",
        "operationId": "getOrderNote",
        "parameters": [
          {
            "in": "path",
            "name": "orderNoteId",
            "description": "Order Note ID to fetch",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int64"
            }
          }
        ],
        "responses": {
          "200": {
            "$ref": "#/components/responses/OrderNote_Get"
          },
          "default": {
            "$ref": "#/components/responses/DefaultErrorResponse"
          }
        }
      }
    }
  },
  "components": {
    "securitySchemes": {
      "bearerAuth": {
        "type": "http",
        "scheme": "bearer"
      }
    },
    "responses": {
      "DefaultErrorResponse": {
        "description": "Default error response",
        "content": {
          "application/json": {
            "schema": {
              "allOf": [
                {
                  "$ref": "#/components/schemas/BaseErrorResponse"
                },
                {
                  "type": "object",
                  "properties": {
                    "error": {
                      "type": "object",
                      "properties": {
                        "details": {
                          "type": "string",
                          "description": "Error details."
                        }
                      }
                    }
                  }
                }
              ]
            }
          }
        }
      },
      "OrderNote_Get": {
        "description": "Return details of an Order Note",
        "content": {
          "application/json": {
            "schema": {
              "allOf": [
                {
                  "$ref": "#/components/schemas/BaseResponse"
                },
                {
                  "type": "object",
                  "properties": {
                    "data": {
                      "$ref": "#/components/schemas/OrderNote"
                    }
                  }
                }
              ]
            }
          }
        }
      }
    },
    "schemas": {
      "User": {
        "type": "object",
        "properties": {
          "id": {
            "type": "integer",
            "example": 123
          },
          "name": {
            "type": "string",
            "example": "John Smith"
          },
          "email": {
            "type": "string",
            "example": "agent@company.com"
          },
          "active": {
            "type": "boolean",
            "example": false
          },
          "username": {
            "type": "string",
            "example": "john_smith"
          },
          "role": {
            "type": "string",
            "example": "agent"
          }
        }
      },
      "OrderNote": {
        "type": "object",
        "properties": {
          "id": {
            "type": "integer"
          },
          "sales_order_id": {
            "type": "integer"
          },
          "user": {
            "$ref": "#/components/schemas/User"
          },
          "text": {
            "type": "string",
            "description": "The note content."
          },
          "attachments": {
            "type": "array",
            "description": "Array of Order Note Attachments",
            "items": {
              "$ref": "#/components/schemas/Base_Attachment"
            }
          },
          "created_at": {
            "type": "string",
            "example": "2023-01-21 14:30:00"
          },
          "last_updated_at": {
            "type": "string",
            "example": "2023-01-21 14:30:00"
          }
        }
      },
      "Base_Attachment": {
        "type": "object",
        "properties": {
          "id": {
            "type": "number"
          },
          "name": {
            "example": "cat.jpg",
            "type": "string"
          },
          "link": {
            "example": "http://s3bucket/attahcment.link",
            "type": "string",
            "nullable": true
          },
          "mime": {
            "example": "image/jpeg",
            "type": "string"
          },
          "attachmentType": {
            "type": "string"
          }
        }
      },
      "BaseResponse": {
        "type": "object",
        "properties": {
          "data": {
            "type": "object"
          }
        }
      },
      "BaseErrorResponse": {
        "type": "object",
        "properties": {
          "error": {
            "type": "object",
            "properties": {
              "httpCode": {
                "type": "integer",
                "description": "Error code",
                "example": 500
              },
              "message": {
                "type": "string",
                "description": "Error short message"
              }
            }
          }
        }
      }
    }
  }
}
```