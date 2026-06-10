> ## Documentation Index
> Fetch the complete documentation index at: https://developers.edesk.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Create Template

Create a new template

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
      "post": {
        "tags": [
          "Templates"
        ],
        "operationId": "createTemplate",
        "summary": "Create Template",
        "description": "Create a new template",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/TemplateRequest"
              }
            }
          }
        },
        "responses": {
          "200": {
            "$ref": "#/components/responses/Template_Get"
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
      "Template_Get": {
        "description": "Return details of a Template",
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
                      "$ref": "#/components/schemas/Template"
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
      "TemplateRequest": {
        "type": "object",
        "required": [
          "name",
          "body_text",
          "template_usage",
          "template_type",
          "active"
        ],
        "properties": {
          "name": {
            "type": "string",
            "description": "The name of the template.",
            "example": "My Template"
          },
          "subject": {
            "type": "string",
            "nullable": true,
            "description": "The subject of the message."
          },
          "body_text": {
            "type": "string",
            "description": "The body of the message."
          },
          "channels": {
            "type": "array",
            "nullable": true,
            "items": {
              "type": "integer",
              "example": [
                1,
                2,
                3
              ],
              "description": "The channel IDs."
            },
            "description": "The channel IDs where template will be used."
          },
          "template_usage": {
            "type": "string",
            "example": "Manual",
            "enum": [
              "Manual",
              "ManualAuto"
            ],
            "description": "The usage of the template."
          },
          "template_type": {
            "type": "array",
            "items": {
              "type": "string",
              "enum": [
                "Consumer",
                "Internal",
                "External"
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
              "description": "Ticket Type for which template will be used."
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
            ],
            "description": "The order status for which template will be used."
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
            "items": {
              "$ref": "#/components/schemas/Template_AiClassificationRequest"
            },
            "nullable": true
          },
          "attachments": {
            "type": "array",
            "nullable": true,
            "items": {
              "$ref": "#/components/schemas/Template_AttachmentRequest"
            }
          },
          "delete_attachments": {
            "type": "array",
            "nullable": true,
            "items": {
              "type": "integer",
              "example": [
                1,
                2,
                3
              ],
              "description": "The attachment IDs to delete."
            }
          }
        }
      },
      "Template_AttachmentRequest": {
        "type": "object",
        "required": [
          "name",
          "url"
        ],
        "properties": {
          "url": {
            "example": "https://test.com/cat.jpg",
            "type": "string",
            "format": "uri"
          },
          "name": {
            "example": "cat.jpg",
            "type": "string",
            "maxLength": 100
          }
        }
      },
      "Template_AiClassificationRequest": {
        "type": "object",
        "required": [
          "classification"
        ],
        "properties": {
          "classification": {
            "type": "string",
            "example": "WrongItem",
            "description": "The AI category for suggested responses.",
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
            "description": "Template will be used for quick reply by AI category",
            "example": true,
            "type": "boolean"
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