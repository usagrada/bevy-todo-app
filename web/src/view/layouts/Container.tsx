import styled from '@emotion/styled';
import React, { VFC, ReactNode } from 'react';

interface Props {
  children: ReactNode;
}
const Container: VFC<Props> = ({ children }: Props) => {
  return <MainComponent>{children}</MainComponent>;
};

const MainComponent = styled.main`
  position: relative;
  padding: 40px 10%;
  background-size: cover;
  background-color: rgba(255, 255, 255, 0.5);
  background-blend-mode: lighten;
  overflow: hidden;
`;

export default Container;
