import {Button, Container, Input, Modal, NextUIProvider, Progress, Spacer, styled, Text} from '@nextui-org/react';
import {useForm} from 'react-hook-form';
import {useCallback, useEffect, useMemo, useState} from 'react';
import {useStatusState} from './hooks/useStatusState';
import ImageLoader from './ImageLoader/ImageLoader';

const ButtonContainer = styled('div', {
  display: 'flex',
  width: '100%',
  justifyContent: 'end',
  alignItems: 'center',
})

const ProgressStyled = styled(Progress, {
  marginRight: '$10',
})

interface FormState {
  address: string
  word: string
}

function App() {
  const {register, handleSubmit} = useForm<FormState>()
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

  const onSubmit = useCallback(handleSubmit((form: FormState) =>
    void wrapPromise(async () => {
      const searchParams = new URLSearchParams()
      searchParams.set('address', form.address)
      searchParams.set('word', form.word)
      console.log(form, file)
      const resp = await fetch(`http://localhost:9300/api/invoke_count_matches?${searchParams}`, {
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
    })()
  ), [wrapPromise, file])
  return (
    <NextUIProvider>
      <Container>
        <Spacer y={2}/>
        <Text h2>
          Hack FEVM demo
        </Text>
        <Spacer y={3.5}/>
        <form onSubmit={onSubmit}>
          <ImageLoader onChange={setFile}/>
          <Spacer y={2}/>
          <Input
            clearable
            bordered
            fullWidth
            labelPlaceholder="Address"
            {...register('address')}
          />
          <Spacer y={2}/>
          <Input
            clearable
            bordered
            fullWidth
            labelPlaceholder="Word"
            {...register('word')}
          />
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
      </Container>
    </NextUIProvider>
  )
}

export default App
