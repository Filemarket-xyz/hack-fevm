import {Container, NextUIProvider, Spacer, Text} from '@nextui-org/react';
import {TextForm} from './TextForm/TextForm';
import {ImageForm} from './ImageForm/ImageForm';

function App() {
  return (
    <NextUIProvider>
      <Container>
        <Spacer y={2}/>
        <Text h2>
          Hack FEVM demo
        </Text>
        <Spacer y={2}/>
        <TextForm/>
        <Spacer y={3}/>
        <ImageForm/>
      </Container>
    </NextUIProvider>
  )
}

export default App
