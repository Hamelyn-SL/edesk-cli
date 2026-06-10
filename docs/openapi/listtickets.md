> ## Documentation Index
> Fetch the complete documentation index at: https://developers.edesk.com/llms.txt
> Use this file to discover all available pages before exploring further.

# List Tickets

Return a list of tickets

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
    "/tickets": {
      "get": {
        "tags": [
          "Tickets"
        ],
        "operationId": "listTickets",
        "summary": "List Tickets",
        "description": "Return a list of tickets",
        "parameters": [
          {
            "in": "query",
            "name": "order_by",
            "description": "Order by field",
            "schema": {
              "type": "string",
              "enum": [
                "id",
                "created_at",
                "last_updated_at"
              ]
            }
          },
          {
            "in": "query",
            "name": "order_direction",
            "description": "Order direction",
            "schema": {
              "type": "string",
              "enum": [
                "asc",
                "desc"
              ]
            }
          },
          {
            "in": "query",
            "description": "Filter by contact ID",
            "name": "filter_contact_id_equals",
            "schema": {
              "type": "integer",
              "format": "int64"
            }
          },
          {
            "in": "query",
            "description": "Filter by channel ID",
            "name": "filter_channel_id_equals",
            "schema": {
              "type": "integer",
              "format": "int64"
            }
          },
          {
            "in": "query",
            "description": "Filter by status",
            "name": "filter_status_equals",
            "schema": {
              "type": "string",
              "enum": [
                "Scheduled",
                "Spam",
                "Unread",
                "Read",
                "Unpriority",
                "Priority",
                "Archived",
                "Open",
                "Pending",
                "Closed"
              ]
            }
          },
          {
            "in": "query",
            "description": "Filter by type",
            "name": "filter_type_equals",
            "schema": {
              "type": "string",
              "enum": [
                "BuyerNotes",
                "Cancellation",
                "Chat",
                "Chatbot",
                "ContactBuyer",
                "ContactForm",
                "DefectiveItemReceived",
                "FeedbackReply",
                "Incident",
                "InvoiceRequest",
                "NegativeFeedback",
                "OfferQuery",
                "OrderAdjustment",
                "OrderClaim",
                "OrderQuery",
                "OrderShippingInquiry",
                "PaymentsQuery",
                "Presales",
                "ProductQuery",
                "PublicMessage",
                "Query",
                "Refund",
                "ResolutionCase",
                "ReturnRequest",
                "ReturnsQuery",
                "SampleQuery",
                "ShippingQuery",
                "SystemMessage",
                "WrongItemReceived"
              ]
            }
          },
          {
            "in": "query",
            "description": "Filter by sales order ID",
            "name": "filter_sales_order_id_equals",
            "schema": {
              "type": "integer",
              "format": "int64"
            }
          },
          {
            "in": "query",
            "description": "Filter by created after date",
            "name": "filter_created_at_gte",
            "schema": {
              "type": "integer",
              "example": "2020-01-01",
              "format": "int64"
            }
          },
          {
            "in": "query",
            "description": "Filter by created before date",
            "name": "filter_created_at_lte",
            "schema": {
              "type": "integer",
              "example": "2023-01-01",
              "format": "int64"
            }
          },
          {
            "in": "query",
            "description": "Filter by last updated after date (YYYY-MM-DD, inclusive start of day in user's timezone)",
            "name": "filter_last_updated_at_gte",
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2026-05-18"
            }
          },
          {
            "in": "query",
            "description": "Filter by last updated before date (YYYY-MM-DD, inclusive end of day in user's timezone)",
            "name": "filter_last_updated_at_lte",
            "schema": {
              "type": "string",
              "format": "date",
              "example": "2026-05-18"
            }
          },
          {
            "in": "query",
            "description": "Filter by owner user ID",
            "name": "filter_owner_user_id_equals",
            "schema": {
              "type": "integer",
              "format": "int64"
            }
          },
          {
            "in": "query",
            "description": "Filter by seller order ID",
            "name": "filter_seller_order_id_equals",
            "schema": {
              "type": "string"
            }
          }
        ],
        "responses": {
          "200": {
            "$ref": "#/components/responses/Ticket_List"
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
      "Ticket_List": {
        "description": "Get the list of tickets from all channels",
        "content": {
          "application/json": {
            "schema": {
              "allOf": [
                {
                  "$ref": "#/components/schemas/BaseResponse_List"
                },
                {
                  "type": "object",
                  "properties": {
                    "data": {
                      "type": "array",
                      "description": "An array of Tickets",
                      "items": {
                        "$ref": "#/components/schemas/Ticket"
                      }
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
      "Ticket": {
        "type": "object",
        "properties": {
          "id": {
            "type": "integer",
            "example": 123
          },
          "subject": {
            "type": "string",
            "example": "subject"
          },
          "type": {
            "type": "string",
            "example": "ProductQuery",
            "description": "Ticket type label",
            "enum": [
              "BuyerNotes",
              "Cancellation",
              "Chat",
              "Chatbot",
              "ContactBuyer",
              "ContactForm",
              "DefectiveItemReceived",
              "FeedbackReply",
              "Incident",
              "InvoiceRequest",
              "NegativeFeedback",
              "OfferQuery",
              "OrderAdjustment",
              "OrderClaim",
              "OrderQuery",
              "OrderShippingInquiry",
              "PaymentsQuery",
              "Presales",
              "ProductQuery",
              "PublicMessage",
              "Query",
              "Refund",
              "ResolutionCase",
              "ReturnRequest",
              "ReturnsQuery",
              "SampleQuery",
              "ShippingQuery",
              "SystemMessage",
              "WrongItemReceived"
            ]
          },
          "channel_id": {
            "type": "integer"
          },
          "status": {
            "type": "string",
            "enum": [
              "Scheduled",
              "Spam",
              "Archived",
              "Open",
              "Pending",
              "Closed"
            ]
          },
          "sales_order_id": {
            "type": "integer"
          },
          "sales_order": {
            "$ref": "#/components/schemas/SalesOrder"
          },
          "external_order_id": {
            "type": "integer"
          },
          "created_at": {
            "type": "number"
          },
          "last_updated_at": {
            "type": "string",
            "nullable": true,
            "example": "2021-05-19 11:05:08",
            "description": "Datetime of the last message on the ticket (null when there are no messages)"
          },
          "owner_user_id": {
            "type": "integer"
          },
          "custom_fields": {
            "type": "array",
            "description": "Array of CustomField items",
            "items": {
              "$ref": "#/components/schemas/CustomField"
            }
          },
          "time_left_to_reply": {
            "type": "number"
          },
          "tags_ids": {
            "type": "array",
            "description": "Array of Tag Ids",
            "items": {
              "type": "integer"
            }
          },
          "contact_id": {
            "type": "integer",
            "description": "Consumer ID"
          },
          "messages_ids": {
            "type": "array",
            "description": "Array of Message Ids",
            "items": {
              "type": "integer"
            }
          },
          "uri": {
            "type": "string",
            "description": "Ticket URL"
          },
          "replies": {
            "type": "integer",
            "description": "Total number of replies"
          },
          "ai_classification": {
            "type": "string",
            "description": "AI classification label"
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
      "BaseResponse_List": {
        "type": "object",
        "properties": {
          "data": {
            "type": "array"
          },
          "paginator": {
            "$ref": "#/components/schemas/Paginator"
          }
        }
      },
      "Paginator": {
        "type": "object",
        "properties": {
          "currentPage": {
            "type": "integer"
          },
          "itemsPerPage": {
            "type": "integer"
          },
          "totalItemsCount": {
            "type": "integer"
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