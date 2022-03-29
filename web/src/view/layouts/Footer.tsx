import React, { VFC } from 'react';
import styled from '@emotion/styled';

const Footer: VFC = () => {
  return <CustomFooter>Footer</CustomFooter>;
};

const CustomFooter = styled.footer`
  height: 60px;
  padding: 10px 10px;
  background-color: #7b90d2;
`;

export default Footer;
