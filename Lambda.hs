module Lambda () where
  
data Value =
    Variable String
  | Application Value Value
  | Lambda String Value
  deriving (Show)

instance Eq Value where
  (Variable s1) == (Variable s2) = s1 == s2
  (Application f1 a1) == (Application f2 a2) = f1 == f2 && a1 == a2
  (Lambda p1 b1) == (Lambda p2 b2) = p1 == p2 && b1 == b2

clone :: Value -> Value
clone (Variable s) = Variable s
clone (Application f a) = Application (clone f) (clone a)
clone (Lambda p b) = Lambda p (clone b)

church :: Int -> Value
church n =
  let body 0 = Variable "x"
      body m = Application (Variable "f") (body (m - 1))
  in Lambda "f" (Lambda "x" (body n))

betaEquiv :: Value -> Value -> Bool
betaEquiv v1 v2 =
  let deBruijnIndex s [] = Nothing
      deBruijnIndex s (x : xs) =
        if x == s
          then Just 0
          else case deBruijnIndex s xs of
            Just i -> Just (i + 1)
            Nothing -> Nothing

      betaEquivWith (Variable s1) (Variable s2) ps1 ps2 =
        case (deBruijnIndex s1 ps1, deBruijnIndex s2 ps2) of
          (Just i, Just j) -> i == j
          (Nothing, Nothing) -> s1 == s2
          _ -> False

      betaEquivWith (Application f1 a1) (Application f2 a2) ps1 ps2 =
        betaEquivWith f1 f2 ps1 ps2 && betaEquivWith a1 a2 ps1 ps2

      betaEquivWith (Lambda p1 b1) (Lambda p2 b2) ps1 ps2 =
        betaEquivWith b1 b2 (p1 : ps1) (p2 : ps2)

  in betaEquivWith v1 v2 [] []

unboundVars :: Value -> [String]
unboundVars v =
  let unboundVarsWith (Variable s) bound =
        if elem s bound
          then []
          else [s]

      unboundVarsWith (Application f a) bound =
        (unboundVarsWith f bound) ++ (unboundVarsWith a bound)

      unboundVarsWith (Lambda p b) bound =
        unboundVarsWith b (p : bound)

  in unboundVarsWith v []

replace :: Value -> String -> Value -> Value
replace (Variable s) t v =
  if s == t
    then v
    else Variable s

replace (Application f a) t v =
  Application (replace f t v) (replace a t v)

replace (Lambda p b) t v =
  if p == t
    then Lambda p b
    else if elem p (unboundVars v)
      then
        let p' = p ++ "_"
            b' = replace b p (Variable p')
        in Lambda p' (replace b' t v)
      else Lambda p (replace b t v)

reduceOne :: Value -> Maybe Value

reduceOne (Variable s) = Nothing

reduceOne (Application (Lambda p b) a) = Just (replace b p a)

reduceOne (Application f a) = case reduceOne f of
  Just f' -> Just (Application f' a)
  Nothing -> case reduceOne a of
    Just a' -> Just (Application f a')
    Nothing -> Nothing

reduceOne (Lambda p b) = case reduceOne b of
  Just b' -> Just (Lambda p b')
  Nothing -> Nothing

reduceN :: Int -> Value -> Value
reduceN 0 v = v
reduceN n v = case reduceOne v of
  Just v' -> reduceN (n - 1) v'
  Nothing -> v

reduceToNormal :: Value -> Value
reduceToNormal v = case reduceOne v of
  Just v' -> reduceToNormal v'
  Nothing -> v
