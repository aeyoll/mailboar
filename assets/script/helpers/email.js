export const parseEmail = (rawEmail) => {
  const regex = /<(.*?)>/;
  let email = rawEmail.match(regex);

  if (typeof email[1] !== 'undefined') {
    email = email[1];
  } else {
    email = '';
  }

  return email;
};
