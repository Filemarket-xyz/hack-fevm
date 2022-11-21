import {Button, Input, Modal, Progress, Spacer, styled, Text} from '@nextui-org/react';
import {useForm} from 'react-hook-form';
import {useCallback, useEffect, useMemo, useState} from 'react';
import {useStatusState} from '../hooks/useStatusState';

// import { JsKeyPair } from "hidden-file-client";

export const ButtonContainer = styled('div', {
  display: 'flex',
  width: '100%',
  justifyContent: 'end',
  alignItems: 'center',
})

export const ProgressStyled = styled(Progress, {
  marginRight: '$10',
})


interface FormState {
  address: string
  word: string
}

export function TextForm() {
  const {register, handleSubmit} = useForm<FormState>()

  const {wrapPromise, statuses: {isLoading, result, error}} = useStatusState<string>()

  // useEffect(() => {
  //   try {
  //     const keypair = new JsKeyPair();
  //     console.log(keypair.private);
  //     console.log(keypair.public); 
  //   } catch (error) {
  //     console.error(error)
  //   }
  // },[])

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
      const resp = await fetch(`http://localhost:9300/api/invoke_count_matches?${searchParams}`, {
        method: 'POST',
      })
      const text = await resp.text()
      if (resp.ok) {
        return text
      } else {
        throw text
      }
    })()
  ), [wrapPromise])
  return (
    <>
      <Text h3>
        Count word matches count
      </Text>
      <Spacer y={2.5}/>
      <form onSubmit={onSubmit}>
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
    </>
  )
}
