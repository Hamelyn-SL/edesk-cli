> ## Documentation Index
> Fetch the complete documentation index at: https://developers.edesk.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Read Tag

Return details of a tag

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
    "/tags/{tagId}": {
      "get": {
        "tags": [
          "Tags"
        ],
        "summary": "Read Tag",
        "description": "Return details of a tag",
        "operationId": "getTag",
        "parameters": [
          {
            "in": "path",
            "name": "tagId",
            "description": "Tag ID to fetch",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int64"
            }
          }
        ],
        "responses": {
          "200": {
            "$ref": "#/components/responses/Tag_Get"
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
      "Tag_Get": {
        "description": "Return details of a Tag",
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
                      "$ref": "#/components/schemas/Tag"
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
      "Tag": {
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
          "active": {
            "type": "boolean",
            "example": true
          },
          "tag_group_id": {
            "type": "integer"
          },
          "color": {
            "type": "string",
            "nullable": true,
            "example": 757575,
            "enum": [
              "757575",
              "F44336",
              "E91E63",
              "9C27B0",
              "673AB7",
              "3F51B5",
              "2196F3",
              "03A9F4",
              "00BCD4",
              "009688",
              "4CAF50",
              "8BC34A",
              "FFA000",
              "FB8C00",
              "FF5722",
              "795548",
              "607D8B",
              "000000",
              "F68843",
              "3BB9FB",
              "8488E7",
              "2C6DEF",
              "FFDF7E",
              "CFAAF8",
              "B6D44C",
              "F480B6",
              "EB3F48",
              "76767B",
              "D682C8"
            ]
          },
          "icon": {
            "type": "string",
            "nullable": true,
            "example": "folder",
            "enum": [
              "folder",
              "heart",
              "hashtag",
              "automobile",
              "ban",
              "bank",
              "bell",
              "bolt",
              "bullhorn",
              "bullseye",
              "calendar",
              "check",
              "close",
              "cog",
              "comment",
              "comments",
              "exclamation",
              "eye",
              "flag",
              "gift",
              "group",
              "globe",
              "info",
              "legal",
              "magic",
              "money",
              "plane",
              "plug",
              "print",
              "recycle",
              "refresh",
              "reply",
              "smile",
              "star",
              "support",
              "tint",
              "tree",
              "user",
              "users",
              "unlock",
              "wheelchair",
              "wrench",
              "euro",
              "gbp",
              "remove",
              "dropbox",
              "truck",
              "dollar",
              "phone",
              "adjust",
              "adn",
              "ambulance",
              "anchor"
            ]
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