> ## Documentation Index
> Fetch the complete documentation index at: https://developers.edesk.com/llms.txt
> Use this file to discover all available pages before exploring further.

# List Templates

Get the list of templates

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
    "/templates": {
      "get": {
        "tags": [
          "Templates"
        ],
        "operationId": "listTemplates",
        "summary": "List Templates",
        "description": "Get the list of templates",
        "responses": {
          "200": {
            "$ref": "#/components/responses/Template_List"
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
      "Template_List": {
        "description": "Get the list of Templates",
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
                      "description": "An array of Templates",
                      "items": {
                        "$ref": "#/components/schemas/Template"
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
      "Template": {
        "type": "object",
        "properties": {
          "id": {
            "type": "integer"
          },
          "name": {
            "type": "string"
          },
          "subject": {
            "type": "string",
            "nullable": true
          },
          "body_text": {
            "type": "string"
          },
          "channels": {
            "type": "array",
            "description": "Array of Channel Ids",
            "nullable": true,
            "items": {
              "type": "integer",
              "example": [
                1,
                2,
                3
              ],
              "description": "The channel IDs."
            }
          },
          "template_usage": {
            "type": "string",
            "example": "Manual",
            "enum": [
              "Manual",
              "ManualAuto",
              "Auto",
              "Rule",
              "OutOfOffice"
            ],
            "description": "The usage of the template."
          },
          "template_type": {
            "type": "array",
            "description": "Array of template types",
            "items": {
              "type": "string",
              "enum": [
                "Consumer",
                "Internal",
                "External",
                "ChatConsumer"
              ]
            },
            "example": [
              "Consumer"
            ]
          },
          "query_type": {
            "type": "array",
            "nullable": true,
            "items": {
              "type": "string",
              "example": [
                "BuyerNotes"
              ],
              "enum": [
                "BuyerNotes",
                "Cancellation",
                "Chat",
                "ContactBuyer",
                "ContactForm",
                "DefectiveItemReceived",
                "FeedbackReply",
                "Incident",
                "InvoiceRequest",
                "NegativeFeedback",
                "OfferQuery",
                "OrderClaim",
                "OrderQuery",
                "PaymentsQuery",
                "Presales",
                "ProductQuery",
                "PublicMessage",
                "Query",
                "ResolutionCase",
                "ReturnRequest",
                "ReturnsQuery",
                "SampleQuery",
                "OrderShippingInquiry",
                "ShippingQuery",
                "SystemMessage",
                "WrongItemReceived"
              ],
              "description": "Ticket Type"
            }
          },
          "order_status": {
            "type": "array",
            "nullable": true,
            "items": {
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
            "example": [
              "OrderReceived"
            ]
          },
          "delivery_date": {
            "type": "string",
            "nullable": true,
            "example": 1,
            "enum": [
              "Within",
              "Outside"
            ]
          },
          "active": {
            "type": "boolean",
            "example": true
          },
          "order_fulfilment": {
            "nullable": true,
            "type": "array",
            "example": [
              "FBA"
            ],
            "items": {
              "type": "string",
              "enum": [
                "FBA",
                "MerchantFulfilled"
              ]
            }
          },
          "created_at": {
            "type": "string",
            "example": "2023-01-21 14:30:00"
          },
          "message_subject": {
            "type": "string",
            "nullable": true,
            "example": "Your order has been shipped"
          },
          "invoice_attached": {
            "type": "boolean",
            "example": true
          },
          "only_use_if_no_replies_yet": {
            "type": "boolean",
            "example": true
          },
          "ai_classification": {
            "$ref": "#/components/schemas/Template_AiClassification"
          },
          "attachments": {
            "type": "array",
            "description": "Array of Template Attachment",
            "items": {
              "$ref": "#/components/schemas/Template_Attachment"
            }
          }
        }
      },
      "Template_Attachment": {
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
          }
        }
      },
      "Template_AiClassification": {
        "type": "object",
        "nullable": true,
        "properties": {
          "classification": {
            "type": "string",
            "example": "WrongItem",
            "enum": [
              "WhereOrder",
              "ShipmentInstructions",
              "PreorderShipmentInquiry",
              "GiftPurchase",
              "FaultyItem",
              "WrongItem",
              "MissingItems",
              "Damaged",
              "DontFit",
              "NotAsDescribed",
              "Warranty",
              "OtherReturn",
              "RefundRequest",
              "CancellationRequest",
              "PaymentIssue",
              "InvoiceRequest",
              "NoResponseNeeded",
              "HappyCustomer",
              "OutOfOffice",
              "OrderIssue",
              "ProductQuery",
              "TrackingCodeRequest",
              "UnhappyCustomer",
              "Replacement",
              "EvidenceReceived"
            ]
          },
          "quick_reply": {
            "type": "boolean"
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