import ImageLoader from '../ImageLoader/ImageLoader';
import {Button, Modal, Spacer, Text} from '@nextui-org/react';
import {useCallback, useEffect, useMemo, useState} from 'react';
import {useStatusState} from '../hooks/useStatusState';
import {ButtonContainer, ProgressStyled} from '../TextForm/TextForm';

export function ImageForm() {
  const [file, setFile] = useState<File>()
  const {wrapPromise, statuses: {isLoading, result, error}} = useStatusState<string>()

  const modalContent = useMemo(() => result || error, [result, error])
  const [modalOpen, setModalOpen] = useState(Boolean(result || error))
  const closeModal = () => setModalOpen(false)
  useEffect(() => {
    if (result || error) {
      setModalOpen(true)
    }
  }, [result, error])

  const onSubmit = useCallback(wrapPromise(async () => {
      const resp = await fetch(`http://localhost:9300/api/create_actor`, {
        method: 'POST',
        headers: file ? {
          'Content-Type': file.type,
          'Content-Disposition': `attachment; filename="${file.name}"`,
        } : undefined,
        body: await file?.arrayBuffer(),
      })
      const text = await resp.text()
      if (resp.ok) {
        return text
      } else {
        throw text
      }
    }), [wrapPromise, file])
  return (
    <>
      <Text h3>Create actor</Text>
      <Spacer y={2}></Spacer>
      <form onSubmit={event => {
        event.preventDefault()
        void onSubmit()
      }}>
        <ImageLoader onChange={setFile}/>
        <Spacer y={2.5}/>
        <ButtonContainer>
          {isLoading && (
            <ProgressStyled
              indeterminated
            />
          )}
          <Button type="submit" disabled={isLoading}>Submit</Button>
        </ButtonContainer>
      </form>
      <Modal
        closeButton
        aria-labelledby="modal-title"
        open={modalOpen}
        onClose={closeModal}
      >
        <Modal.Header>
          <Text id="modal-title" size={18}>
            {error ? 'An error occurred' : 'Success!'}
          </Text>
        </Modal.Header>
        <Modal.Body>
          <Text color={error ? 'error' : 'success'}>
            {modalContent}
          </Text>
        </Modal.Body>
        <Modal.Footer>
          <Button auto onClick={closeModal}>
            Ok
          </Button>
        </Modal.Footer>
      </Modal>
    </>
  )
}
