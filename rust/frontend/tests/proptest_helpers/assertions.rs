use chroma_frontend::Frontend;
use chroma_types::{are_metadatas_close_to_equal, GetResponse, QueryResponse};

pub fn check_get_responses_are_close_to_equal(
    mut reference: GetResponse,
    mut received: GetResponse,
) {
    reference.sort_by_ids();
    received.sort_by_ids();

    assert_eq!(
        reference.ids, received.ids,
        "Expected IDs {:?} to be equal to {:?}",
        reference.ids, received.ids
    );
    // todo: use close check
    assert_eq!(
        reference.embeddings, received.embeddings,
        "Expected embeddings {:?} to be equal to {:?} (IDs: {:?})",
        reference.embeddings, received.embeddings, reference.ids
    );
    assert_eq!(
        reference.documents, received.documents,
        "Expected documents {:?} to be equal to {:?} (IDs: {:?})",
        reference.documents, received.documents, reference.ids
    );
    assert_eq!(
        reference.metadatas.is_none(),
        received.metadatas.is_none(),
        "Expected {:?} to be equal to {:?}",
        reference.metadatas,
        received.metadatas
    );

    if let Some(reference_metadatas) = reference.metadatas.as_ref() {
        if let Some(received_metadatas) = received.metadatas.as_ref() {
            assert_eq!(
                reference_metadatas.len(),
                received_metadatas.len(),
                "Expected {:?} to be equal to {:?}",
                reference,
                received
            );
            for i in 0..reference_metadatas.len() {
                let reference = &reference_metadatas[i];
                let received = &received_metadatas[i];

                match (reference.is_some(), received.is_some()) {
                    (true, false) => {
                        assert!(
                            false,
                            "Expected that metadata at index {} is Some(..). Expected {:?}",
                            i, reference
                        );
                    }
                    (false, true) => {
                        assert!(
                            false,
                            "Expected that metadata at index {} is None, but got {:?}",
                            i, received
                        );
                    }
                    _ => {}
                }

                if let Some(reference) = reference {
                    if let Some(received) = received {
                        assert!(
                            are_metadatas_close_to_equal(reference, received),
                            "Expected {:?} to be equal to {:?}",
                            reference,
                            received
                        );
                    }
                }
            }
        }
    }
}

// todo: check distances
pub async fn check_query_responses_are_close_to_equal(
    mut reference: QueryResponse,
    mut received: QueryResponse,
    reference_frontend: &Frontend,
) {
    // todo: assert that responses are originally sorted by distance

    reference.sort_by_ids();
    received.sort_by_ids();

    assert_eq!(
        received.ids, reference.ids,
        "Got IDs {:?}, expected {:?}",
        received.ids, reference.ids
    );
    assert_eq!(
        reference.embeddings, received.embeddings,
        "Expected {:?} to be equal to {:?}",
        reference.embeddings, received.embeddings
    );
    assert_eq!(
        reference.documents, received.documents,
        "Expected {:?} to be equal to {:?}",
        reference.documents, received.documents
    );
    assert_eq!(
        reference.metadatas.is_none(),
        received.metadatas.is_none(),
        "Expected {:?} to be equal to {:?}",
        reference.metadatas,
        received.metadatas
    );

    if let Some(reference_metadatas_list) = reference.metadatas.as_ref() {
        if let Some(received_metadatas_list) = received.metadatas.as_ref() {
            assert_eq!(
                reference_metadatas_list.len(),
                received_metadatas_list.len(),
                "Expected {:?} to be equal to {:?}",
                reference_metadatas_list.len(),
                received_metadatas_list.len()
            );
            for i in 0..reference_metadatas_list.len() {
                let reference_metadatas = &reference_metadatas_list[i];
                let received_metadatas = &received_metadatas_list[i];

                assert_eq!(
                    reference_metadatas.len(),
                    received_metadatas.len(),
                    "Expected {:?} to be equal to {:?}",
                    reference_metadatas,
                    received_metadatas
                );

                for i in 0..reference_metadatas.len() {
                    let reference = &reference_metadatas[i];
                    let received = &received_metadatas[i];

                    match (reference.is_some(), received.is_some()) {
                        (true, false) => {
                            assert!(
                                false,
                                "Expected that metadata at index {} is Some(..). Expected {:?}",
                                i, reference
                            );
                        }
                        (false, true) => {
                            assert!(
                                false,
                                "Expected that metadata at index {} is None, but got {:?}",
                                i, received
                            );
                        }
                        _ => {}
                    }

                    if let Some(reference) = reference {
                        if let Some(received) = received {
                            assert!(
                                are_metadatas_close_to_equal(reference, received),
                                "Expected {:?} to be equal to {:?}",
                                reference,
                                received
                            );
                        }
                    }
                }
            }
        }
    }
}
