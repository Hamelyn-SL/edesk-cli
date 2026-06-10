> ## Documentation Index
> Fetch the complete documentation index at: https://developers.edesk.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Update Sales Order

Update a sales order

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
    "/sales-orders/{salesOrderId}": {
      "put": {
        "tags": [
          "Sales Orders"
        ],
        "summary": "Update Sales Order",
        "description": "Update a sales order",
        "operationId": "updateSalesOrder",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/SalesOrderUpdateRequest"
              }
            }
          }
        },
        "parameters": [
          {
            "in": "path",
            "name": "salesOrderId",
            "description": "SalesOrder ID to update",
            "required": true,
            "schema": {
              "type": "integer",
              "format": "int64"
            }
          }
        ],
        "responses": {
          "200": {
            "$ref": "#/components/responses/SalesOrder_Get"
          },
          "400": {
            "$ref": "#/components/responses/ValidationErrorResponse"
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
      "ValidationErrorResponse": {
        "description": "Validation error response",
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
                          "type": "object",
                          "additionalProperties": {
                            "type": "object",
                            "properties": {
                              "errorCode": {
                                "$ref": "#/components/schemas/ErrorCodes"
                              }
                            }
                          }
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
      "SalesOrder_Get": {
        "description": "Return details of a Sales Order",
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
                      "$ref": "#/components/schemas/SalesOrder"
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
      "CustomFieldRequest": {
        "type": "object",
        "required": [
          "name",
          "value"
        ],
        "properties": {
          "name": {
            "type": "string",
            "example": "John"
          },
          "value": {
            "type": "string"
          }
        }
      },
      "CustomField": {
        "type": "object",
        "properties": {
          "id": {
            "type": "integer",
            "example": 123
          },
          "name": {
            "type": "string",
            "example": "John"
          },
          "value": {
            "type": "string"
          }
        }
      },
      "SalesOrder": {
        "type": "object",
        "properties": {
          "id": {
            "type": "integer",
            "example": 123
          },
          "channel_id": {
            "type": "integer"
          },
          "seller_order_id": {
            "type": "string",
            "description": "the external order ID",
            "example": "123_XYZ"
          },
          "status": {
            "type": "string",
            "enum": [
              "OrderReceived",
              "PaymentReceived",
              "PaymentRejected",
              "PaymentAccepted",
              "OrderShipped",
              "InTransit",
              "Delivered",
              "Canceled",
              "Returned",
              "Hold"
            ]
          },
          "order_items": {
            "type": "array",
            "description": "Array of Sales Order Items",
            "items": {
              "$ref": "#/components/schemas/SalesOrder_Item"
            }
          },
          "shipping_amount": {
            "type": "number"
          },
          "total_amount": {
            "type": "number"
          },
          "ship_to": {
            "$ref": "#/components/schemas/Address",
            "nullable": true
          },
          "bill_to": {
            "$ref": "#/components/schemas/Address",
            "nullable": true
          },
          "contact_id": {
            "type": "integer"
          },
          "tracking_codes": {
            "type": "array",
            "nullable": true,
            "description": "Array of Sales Order Tracking items",
            "items": {
              "$ref": "#/components/schemas/SalesOrder_Tracking"
            }
          },
          "created_at": {
            "type": "string",
            "example": "2023-01-21 14:30:00"
          },
          "last_updated_at": {
            "type": "string",
            "nullable": true,
            "example": "2023-01-22 09:15:00",
            "description": "Latest of ordered_at, payment_received_at, payment_rejected_at, dispatched_at, cancelled_at"
          },
          "ticket_id": {
            "type": "number",
            "nullable": true
          },
          "order_notes_id": {
            "type": "array",
            "nullable": true,
            "items": {
              "type": "number"
            }
          },
          "sales_order_delivery_dates": {
            "$ref": "#/components/schemas/SalesOrder_Event",
            "nullable": true
          },
          "order_created_at": {
            "type": "string",
            "nullable": true
          },
          "order_shipped_at": {
            "type": "string",
            "nullable": true
          },
          "delivery_type": {
            "type": "string",
            "nullable": true
          },
          "custom_fields": {
            "type": "array",
            "description": "Array of CustomField items",
            "items": {
              "$ref": "#/components/schemas/CustomField"
            }
          }
        }
      },
      "SalesOrder_Tracking": {
        "type": "object",
        "properties": {
          "tracking_code": {
            "type": "string"
          },
          "tracking_link": {
            "type": "string",
            "example": "https://carrier.com/track/123456789"
          },
          "tracking_carrier_name": {
            "type": "string"
          }
        }
      },
      "SalesOrder_TrackingRequest": {
        "type": "object",
        "required": [
          "tracking_code",
          "tracking_carrier_name"
        ],
        "properties": {
          "tracking_code": {
            "type": "string",
            "example": "A1234567890",
            "description": "The unique tracking code as a string."
          },
          "tracking_carrier_link": {
            "type": "string",
            "example": "http://carrier.com/track/(tracking_code)",
            "description": "Tracking link with the (tracking_code) placeholder. You SHOULD NOT change the placeholder with the actual tracking code."
          },
          "tracking_carrier_name": {
            "type": "string",
            "description": "Name of the carrier. Max length 64 characters",
            "example": "USPS"
          }
        }
      },
      "SalesOrder_Item": {
        "type": "object",
        "properties": {
          "id": {
            "type": "integer",
            "description": "Unique id of the sales order item"
          },
          "product": {
            "$ref": "#/components/schemas/ProductChannel"
          },
          "quantity": {
            "type": "integer",
            "description": "Product quantity"
          },
          "end_price": {
            "type": "number"
          }
        }
      },
      "SalesOrder_Event": {
        "type": "object",
        "properties": {
          "expected_delivery_from": {
            "type": "string",
            "nullable": true,
            "description": "Expected delivery date from"
          },
          "expected_delivery_to": {
            "type": "string",
            "nullable": true,
            "description": "Expected delivery date to"
          }
        }
      },
      "Address": {
        "type": "object",
        "properties": {
          "name": {
            "type": "string",
            "example": "John"
          },
          "line_1": {
            "type": "string"
          },
          "line_2": {
            "nullable": true,
            "type": "string"
          },
          "city": {
            "type": "string"
          },
          "state": {
            "nullable": true,
            "type": "string"
          },
          "county": {
            "nullable": true,
            "type": "string"
          },
          "country": {
            "type": "string"
          },
          "postcode": {
            "type": "string"
          },
          "country_name": {
            "type": "string"
          }
        }
      },
      "SalesOrderUpdateRequest": {
        "type": "object",
        "properties": {
          "channel_id": {
            "type": "integer"
          },
          "status": {
            "type": "string",
            "enum": [
              "OrderReceived",
              "PaymentReceived",
              "PaymentRejected",
              "PaymentAccepted",
              "OrderShipped",
              "InTransit",
              "Delivered",
              "Canceled",
              "Returned",
              "Hold"
            ]
          },
          "currency": {
            "type": "string",
            "description": "Currency code (USD, EUR, etc.)",
            "example": "USD"
          },
          "order_items": {
            "type": "array",
            "description": "Array of Sales Order Items",
            "items": {
              "$ref": "#/components/schemas/SalesOrder_ItemUpdateRequest"
            }
          },
          "shipping_amount": {
            "type": "number",
            "example": 0
          },
          "ship_to": {
            "$ref": "#/components/schemas/AddressRequest"
          },
          "bill_to": {
            "$ref": "#/components/schemas/AddressRequest"
          },
          "contact_id": {
            "type": "integer",
            "description": "Consumer Id"
          },
          "tracking_codes": {
            "type": "array",
            "description": "Array of Tracking codes",
            "items": {
              "$ref": "#/components/schemas/SalesOrder_TrackingRequest"
            }
          },
          "sales_order_delivery_dates": {
            "$ref": "#/components/schemas/SalesOrder_EventRequest"
          },
          "order_created_at": {
            "type": "string",
            "format": "date-time",
            "nullable": true,
            "example": "2023-01-21 14:30:00",
            "description": "Date and time"
          },
          "order_shipped_at": {
            "type": "string",
            "format": "date-time",
            "nullable": true,
            "example": "2023-01-21 14:30:00",
            "description": "Order shipped date and time"
          },
          "delivery_type": {
            "type": "string",
            "description": "Specifies the shipment service level category.",
            "enum": [
              "Expedited",
              "NextDay",
              "SecondDay",
              "Standard"
            ]
          },
          "custom_fields": {
            "type": "array",
            "description": "Array of CustomField items",
            "items": {
              "$ref": "#/components/schemas/CustomFieldRequest"
            }
          }
        }
      },
      "SalesOrder_ItemUpdateRequest": {
        "type": "object",
        "required": [
          "id",
          "sku",
          "title",
          "quantity",
          "item_amount",
          "shipping_amount"
        ],
        "properties": {
          "id": {
            "type": "integer",
            "description": "Unique id of the sales order item"
          },
          "sku": {
            "type": "string"
          },
          "title": {
            "type": "string"
          },
          "quantity": {
            "type": "integer",
            "description": "Product quantity"
          },
          "item_amount": {
            "type": "number"
          },
          "shipping_amount": {
            "type": "number"
          },
          "product_image_url": {
            "type": "string",
            "example": "https://test.com/cat.jpg"
          },
          "brand": {
            "type": "string"
          },
          "dimensions": {
            "type": "string",
            "description": "Height x length x width, in centimeters",
            "example": "10x20x30"
          },
          "weight": {
            "type": "string"
          }
        }
      },
      "SalesOrder_EventRequest": {
        "type": "object",
        "properties": {
          "expected_delivery_from": {
            "type": "string",
            "nullable": true,
            "example": "2023-01-15 08:00:00",
            "description": "Expected delivery date from"
          },
          "expected_delivery_to": {
            "type": "string",
            "format": "date-time",
            "nullable": true,
            "example": "2023-01-30 14:21:00",
            "description": "Expected delivery date to"
          }
        }
      },
      "AddressRequest": {
        "type": "object",
        "required": [
          "name",
          "line_1",
          "city",
          "country",
          "postcode"
        ],
        "properties": {
          "name": {
            "type": "string",
            "example": "John"
          },
          "line_1": {
            "type": "string"
          },
          "line_2": {
            "type": "string",
            "nullable": true
          },
          "city": {
            "type": "string"
          },
          "state": {
            "type": "string",
            "nullable": true
          },
          "county": {
            "type": "string",
            "nullable": true
          },
          "country": {
            "type": "string",
            "example": "US",
            "description": "ISO 3166-1 alpha-2 country code"
          },
          "postcode": {
            "type": "string"
          }
        }
      },
      "ProductChannel": {
        "type": "object",
        "properties": {
          "id": {
            "type": "number",
            "example": 123
          },
          "sku": {
            "type": "string",
            "description": "Product SKU"
          },
          "title": {
            "type": "string",
            "description": "Product name"
          },
          "brand": {
            "type": "string"
          },
          "dimensions": {
            "description": "height x length x width, in centimeters",
            "type": "string",
            "nullable": true
          },
          "weight": {
            "description": "Product weight, in grams",
            "type": "number"
          },
          "product_image_url": {
            "type": "string",
            "description": "Link to product image",
            "example": "https://cdn.xsellco.com/images/123.png"
          },
          "marketplace_link": {
            "type": "string",
            "description": "Link to product on marketplace"
          },
          "price": {
            "type": "number",
            "description": "The price of this product"
          },
          "currency": {
            "type": "string",
            "description": "The currency of the price of this product"
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
      },
      "ErrorCodes": {
        "type": "integer",
        "description": "Error codes for each validation type",
        "enum": [
          4001,
          4002,
          4003,
          4004,
          4005,
          4006,
          4007,
          4008,
          4009,
          4010,
          4011,
          4012,
          4013,
          4014,
          4015,
          4016,
          4017,
          4018,
          4019,
          4020,
          4021,
          4022
        ]
      }
    }
  }
}
```