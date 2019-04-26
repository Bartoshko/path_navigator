error_chain! {
    errors {
        InvalidParameter { description("invalid parameter") }
        DataItemIncomplete { description("data item is incomplete") }
        DataItemIncorrect { description("data set is incorrect") }
    }
}
